#include "includes/core.slang"
#include "includes/sobol_burley.slang"

void AdjustSamplePosition(Surface surface, uint primitiveID, inout float3 visibility)
{
    // todo encode smooth position

    // bool smoothP = UseSmoothPosition(surface);

    // float3 positionWS = smoothP ? surface.smoothP : surface.P;
    // positionWS = RayOffset(positionWS, surface.Ng);

    uint i0 = Indices[primitiveID * 3 + 0];
    uint i1 = Indices[primitiveID * 3 + 1];
    uint i2 = Indices[primitiveID * 3 + 2];

    float3 p0 = Vertices[i0].GetPosition();
    float3 p1 = Vertices[i1].GetPosition();
    float3 p2 = Vertices[i2].GetPosition();
    float2 uv0 = Vertices[i0].GetUV();
    float2 uv1 = Vertices[i1].GetUV();
    float2 uv2 = Vertices[i2].GetUV();

    // some ideas from https://ndotl.wordpress.com/2018/08/29/baking-artifact-free-lightmaps/
    float3 edge1 = p1 - p0;
    float3 edge2 = p2 - p0;
    float2 duv1 = uv1 - uv0;
    float2 duv2 = uv2 - uv0;

    float det = duv1.x * duv2.y - duv1.y * duv2.x;
    float invDet = 1.0 / (abs(det) + 1e-9);

    float3 dPdU = (edge1 * duv2.y - edge2 * duv1.y) * invDet;
    float3 dPdV = (edge2 * duv1.x - edge1 * duv2.x) * invDet;

    float3 worldTexelU = dPdU;
    float3 worldTexelV = dPdV;

    float rayLength = (length(worldTexelU) + length(worldTexelV)) * sqrt(2.0); // * 0.5;

    float3 u = normalize(worldTexelU - dot(worldTexelU, surface.Ng) * surface.Ng);
    float3 v = normalize(worldTexelV - dot(worldTexelV, surface.Ng) * surface.Ng);
    float3 rayDirs[4] = {u, -u, v, -v};

    Ray leakRay;
    leakRay.P = RayOffset(surface.P, surface.Ng);
    leakRay.tMin = 0;
    leakRay.tMax = rayLength;
    Hit hit;

    [loop] for (int i = 0; i < 4; i++)
    {
        leakRay.D = rayDirs[i];

        if (TraceRay(leakRay, hit))
        {
            if (!hit.frontFace)
            {
                // Surface hitSurf = Surface::Get(hit.primitiveID, hit.bary, true);
                // positionWS = RayOffset(positionWS + leakRay.D * leakHit.t, hitSurf.Ng);
                visibility.xy = hit.bary.yx; // flip because we hit the back face
                visibility.z = asuint(hit.primitiveID);
                break;
            }
        }
    }
}

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint compactedCount;
    uint sampleIndex;
    uint maxSamples;
    uint lightCount;
};

[shader("compute")]
    [numthreads(64, 1, 1)] void
    main(uint3 threadID : SV_DispatchThreadID)
{
    uint index = threadID.x;

    if (index >= Push.compactedCount)
    {
        return;
    }

    float3 visibility = float3(CompactedVisibility[index * 3 + 0],
                               CompactedVisibility[index * 3 + 1],
                               CompactedVisibility[index * 3 + 2]);

    float2 bary = visibility.xy;
    uint primitiveID = asuint(visibility.z);

    Surface surface = Surface::Get(primitiveID, bary, true);

    AdjustSamplePosition(surface, primitiveID, visibility);

    CompactedVisibility[index * 3 + 0] = visibility.x;
    CompactedVisibility[index * 3 + 1] = visibility.y;
    CompactedVisibility[index * 3 + 2] = visibility.z;
}
