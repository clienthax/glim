#include "includes/core.slang"
#include "includes/sobol_burley.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint width;
    uint height;
    uint sampleIndex;
    uint maxSamples;

    uint lightCount;
    uint pad0;
    uint pad1;
    uint pad2;
};

[shader("compute")]
    [numthreads(8, 8, 1)] void
    main(uint3 threadID : SV_DispatchThreadID)
{
    uint2 coords = threadID.xy;
    uint2 resolution = uint2(Push.width, Push.height);

    if (coords.x >= resolution.x || coords.y >= resolution.y)
    {
        return;
    }

    float4 visibility = VisibilityBuffer[coords];

    if (all(visibility == 0))
    {
        return;
    }

    // uint primitiveID = asuint(visibility.z);
    // float2 bary = visibility.xy;
    // bool useSmoothP = (encoded & (1 << 1)) != 0;

    // Surface surface = Surface::Get(primitiveID, bary);

    // float3 positionWS = RayOffset(surface.P, surface.Ng);
    // float3 normalWS = surface.N;

    float3 positionWS = visibility.xyz;
    float3 normalWS = UnpackNormalOctahedron(visibility.w);

    uint sampleIndex = Push.sampleIndex;
    uint sampleCount = Push.maxSamples;
    uint lightCount = Push.lightCount;

    SamplingPattern pattern = SamplingPattern::BlueNoiseFirst;
    uint pixelSeed = RNGPixelInit(coords, pattern);
    uint sobolIndexMask = ~0u;
    uint3 indexing = BlueNoiseIndexing(pixelSeed, sampleIndex, sobolIndexMask, sampleCount, pattern);

    // todo temp
    // if (sampleIndex == 0)
    // {
    //     visibility = AdjustSamplePositions(surface, visibility);
    //     VisibilityBuffer[coords] = visibility;
    //     encoded = asuint(visibility.a);

    //     if (encoded == 0)
    //     {
    //         return;
    //     }

    //     primitiveID = asuint(visibility.z);
    //     bary = visibility.xy;
    //     useSmoothP = (encoded & (1 << 1)) != 0;
    // }

    // if (useSmoothP)
    // {
    //     surface.P = surface.smoothP;
    // }

    float2 xiLight = SobolBurleySample2D(indexing.x, 0, indexing.y, indexing.z);

    float4 color = float4(0.0, 0.0, 0.0, 1.0);
    float3 throughput = float3(1.0);

    for (uint lightIndex = 0; lightIndex < lightCount; lightIndex++)
    {
        float3 L;
        color.rgb += ShadeLightDirect(Lights[lightIndex], positionWS, normalWS, xiLight, throughput, L);
    }

    float2 xiBounce = SobolBurleySample2D(indexing.x, 1, indexing.y, indexing.z);

    Ray bounceRay;
    bounceRay.P = positionWS;
    bounceRay.D = RandomDirectionInHemisphere(normalWS, xiBounce);
    bounceRay.tMin = 0;
    bounceRay.tMax = RAY_MAX;

    Hit bounceHit;
    if (TraceRay(bounceRay, bounceHit))
    {
        Surface surface = Surface::Get(bounceHit.primitiveID, bounceHit.bary, bounceHit.frontFace);

        if (!bounceHit.frontFace)
        {
            if (!surface.backfaceGI)
            {
                color.a = 0;
            }
        }

        float3 bounceEmission = SampleEmission(surface.uv, surface.lightmapGroup).rgb;
        color.rgb += bounceEmission * color.a;
    }

    float4 previous = LightmapDiffuse[coords];
    AccumulateColor(previous, sampleIndex, color);
    LightmapDiffuse[coords] = previous;
}
