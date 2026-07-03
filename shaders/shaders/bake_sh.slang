#define LIGHTPROBE_SH

#include "includes/core.slang"
#include "includes/sobol_burley.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint lightCount;
    uint maxSamples;
    uint sampleIndex;
    uint probeCount;

    float probesRadius;
    uint pad0;
    uint pad1;
    uint pad2;
};

[shader("compute")]
[numthreads(64, 1, 1)]
void main(uint3 threadID: SV_DispatchThreadID)
{
    uint probeIndex = threadID.x;

    if (probeIndex >= Push.probeCount)
    {
        return;
    }

    SHProbeL2 probe = SHProbes[probeIndex];
    float3 probePosition = probe.position;

    uint sampleIndex = Push.sampleIndex;
    uint sampleCount = Push.maxSamples;
    uint lightCount = Push.lightCount;

    float sampleScale = 1.0 / float(sampleCount);

    uint sobolIndexMask = ~0u;
    SamplingPattern pattern = SamplingPattern::BlueNoisePure;
    uint pixelSeed = RNGPixelInit(uint2(probeIndex.x, 0), pattern);
    uint3 indexing = BlueNoiseIndexing(pixelSeed, sampleIndex, sobolIndexMask, sampleCount, pattern);
    uint dimension = 0;

    float2 xiLight = SobolBurleySample2D(indexing.x, 0, indexing.y, indexing.z);

    float probesRadius = Push.probesRadius;
    if (probesRadius > 0.0)
    {
        probePosition = RandomPointOnSphere(probePosition, probesRadius, xiLight);
    }

    float3 throughput = float3(1.0);

    for (uint lightIndex = 0; lightIndex < lightCount; lightIndex++)
    {
        float3 L;
        float3 color = ShadeLightDirectSH(Lights[lightIndex], probePosition, xiLight, throughput, L);
        AccumulateL2(probe, sampleIndex, color * sampleScale, L);
    }

    float2 xiBounce = SobolBurleySample2D(indexing.x, 1, indexing.y, indexing.z);

    Ray bounceRay;
    bounceRay.P = probePosition;
    bounceRay.D = RandomDirectionSphere(xiBounce);
    bounceRay.tMin = 0;
    bounceRay.tMax = RAY_MAX;

    Hit bounceHit;
    if (TraceRay(bounceRay, bounceHit))
    {
        Surface surface = Surface::Get(bounceHit.primitiveID, bounceHit.bary, bounceHit.frontFace);

        bool backface = !bounceHit.frontFace && !surface.backfaceGI;

        if (!backface)
        {
            float3 bounceEmission = SampleEmission(surface.uv, surface.lightmapGroup).rgb;
            AccumulateL2(probe, sampleIndex, bounceEmission.rgb * 4 * sampleScale, bounceRay.D);

            float3 bounceAlbedo = SampleAlbedo(surface.uv, surface.lightmapGroup).rgb;
            // todo why flipped
            float3 previousDiffuse = SamplePreviousDiffuse(float2(surface.uv.x, 1.0 - surface.uv.y), surface.lightmapGroup).rgb;
            AccumulateL2(probe, sampleIndex, previousDiffuse * bounceAlbedo * 4 * sampleScale, bounceRay.D);
        }
    }

    SHProbes[probeIndex] = probe;
}
