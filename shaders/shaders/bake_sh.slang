#include "includes/bake_core.slang"
#include "includes/sobol_burley.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    Vertex *vertices;
    uint *indices;

    Light *lights;
    uint lightCount;
    uint maxSamples;

    uint sampleIndex;
    uint probeCount;
    uint pad0;
    uint bounceCount;
};

void ShadeLight(inout Spectrum spectrum, Light light, Surface surface, float2 xi, float3 throughput)
{
    Ray shadowRay;
    shadowRay.P = RayOffset(surface.P, surface.Ng);
    shadowRay.tMin = 0;

    float falloff;
    float3 L;
    if (light.type == LightType::Directional)
    {
        L = GetDirectionalLightAngle(light.direction, light.ShadowRangeOrAngle, xi);
        falloff = 1.0;

        shadowRay.tMax = RAY_MAX;
    }
    else
    {
        L = RandomPointOnSphere(light.position, light.ShadowRangeOrAngle, xi);

        float3 positionToLight = L - surface.P;
        L = normalize(positionToLight);

        float distanceSquare = dot(positionToLight, positionToLight);

        float rangeSq = light.range * light.range;

// todo: add falloff type constant
#if 1
        falloff = GetDefaultUnityAttenuation(distanceSquare, 1.0 / rangeSq);
#else
        falloff = GetLightFalloff(distanceSquare, 1.0 / rangeSq);
#endif

        shadowRay.tMax = sqrt(distanceSquare);
    }

    float cosTheta = saturate(dot(surface.N, L));

    shadowRay.D = L;

    float3 color = cosTheta * falloff * light.color * throughput;

    [branch] if (any(color > 0.0))
    {
        if (TraceShadowRay(shadowRay))
        {
            color = 0.0;
        }
    }

    SpectrumAdd(spectrum, color, L);
}

void ShadeLightDirect(inout Spectrum spectrum, Light light, float3 position, float2 xi, float3 throughput)
{
    Ray shadowRay;
    shadowRay.P = position;
    shadowRay.tMin = 0;

    float falloff;
    float3 L;
    if (light.type == LightType::Directional)
    {
        L = GetDirectionalLightAngle(light.direction, light.ShadowRangeOrAngle, xi);
        falloff = 1.0;

        shadowRay.tMax = RAY_MAX;
    }
    else
    {
        L = RandomPointOnSphere(light.position, light.ShadowRangeOrAngle, xi);

        float3 positionToLight = L - position;
        L = normalize(positionToLight);

        float distanceSquare = dot(positionToLight, positionToLight);

        float rangeSq = light.range * light.range;

// todo: add falloff type constant
#if 1
        falloff = GetDefaultUnityAttenuation(distanceSquare, 1.0 / rangeSq);
#else
        falloff = GetLightFalloff(distanceSquare, 1.0 / rangeSq);
#endif

        shadowRay.tMax = sqrt(distanceSquare);
    }

    shadowRay.D = L;

    float3 color = falloff * light.color * throughput;

    [branch] if (any(color > 0.0))
    {
        if (TraceShadowRay(shadowRay))
        {
            color = 0.0;
        }
    }

    SpectrumAdd(spectrum, color, L);
}

[shader("compute")]
    [numthreads(64, 1, 1)] void
    main(uint3 threadID : SV_DispatchThreadID)
{
    uint probeIndex = threadID.x;

    if (probeIndex >= Push.probeCount)
    {
        return;
    }

    SHProbe probe = SHProbes[probeIndex];
    float3 probePosition = float3(probe.positionX, probe.positionY, probe.positionZ);

    Spectrum spectrum = SpectrumZero();
    float3 throughput = float3(1.0);

    // todo backface gi
    bool backfaceGI = false;
    uint sampleIndex = Push.sampleIndex;
    uint sampleCount = Push.maxSamples;
    uint bounceCount = Push.bounceCount;
    uint lightCount = Push.lightCount;
    uint dimension = 0;
    // todo set constant
    uint shuffledIndexMask = reversebits(NextPowerOfTwo(sampleCount - 1) - 1);
    // uint shuffledIndexMask = ~0u;

    uint pixelSeed = HashPcg3d(probeIndex.xxx).x;

    float2 xiLight = SobolBurleySample2D(sampleIndex, dimension++, pixelSeed, shuffledIndexMask);

    for (uint lightIndex = 0; lightIndex < lightCount; lightIndex++)
    {
        ShadeLightDirect(spectrum, Push.lights[lightIndex], probePosition, xiLight, throughput);
    }

    float2 xiBounce0 = SobolBurleySample2D(sampleIndex, dimension++, pixelSeed, shuffledIndexMask);

    Ray bounceRay;
    bounceRay.P = probePosition;
    bounceRay.D = RandomDirectionSphere(xiBounce0);
    bounceRay.tMin = 0;
    bounceRay.tMax = RAY_MAX;

    // bounces
    for (uint bounceIndex = 0; bounceIndex < bounceCount; bounceIndex++)
    {
        Hit bounceHit;
        if (TraceRay(bounceRay, bounceHit))
        {
            Surface surfaceBounce = Surface::Get(Push.indices, Push.vertices, bounceHit.primitiveID, bounceHit.bary);

            if (!bounceHit.frontFace)
            {
                if (!backfaceGI)
                {
                    break;
                }

                surfaceBounce.N = -surfaceBounce.N;
                surfaceBounce.Ng = -surfaceBounce.Ng;
            }

            float3 bounceEmission = SampleEmission(surfaceBounce.uv, surfaceBounce.lightmapGroup).rgb;
            float3 bounceAlbedo = SampleAlbedo(surfaceBounce.uv, surfaceBounce.lightmapGroup).rgb;

            SpectrumAdd(spectrum, bounceEmission * throughput * float.getPi(), bounceRay.D);

            throughput *= bounceAlbedo;

            float2 xiLight = SobolBurleySample2D(sampleIndex, dimension++, pixelSeed, shuffledIndexMask);
            float2 xiBounce = SobolBurleySample2D(sampleIndex, dimension++, pixelSeed, shuffledIndexMask);

            for (uint lightIndex = 0; lightIndex < lightCount; lightIndex++)
            {
                ShadeLight(spectrum, Push.lights[lightIndex], surfaceBounce, xiLight, throughput);
            }

            bounceRay.P = RayOffset(surfaceBounce.P, surfaceBounce.Ng);
            bounceRay.D = RandomDirectionInHemisphere(surfaceBounce.N, xiBounce);
        }
        else
        {
            break;
        }
    }

    AccumulateSH(SHProbes, probeIndex, spectrum);
}
