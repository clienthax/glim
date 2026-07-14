#include "includes/core.slang"
#include "includes/util.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint2 resolution;
    uint offset;
    uint compactedCount;
};

[numthreads(8, 8, 1)] void main(uint3 threadID : SV_DispatchThreadID)
{
    uint2 coords = threadID.xy;

    if (coords.x >= Push.resolution.x || coords.y >= Push.resolution.y)
    {
        return;
    }

    uint index = InterleaveWithZero(coords.x) |
                 (InterleaveWithZero(coords.y) << 1);

    uint word = index >> 5;
    uint bit = index & 31;

    uint2 entry = CompactionBuffer[Push.offset + word];

    uint linearIndex = coords.y * Push.resolution.x + coords.x;

    if ((entry.x & (1u << bit)) == 0)
    {
        DecompactTarget[linearIndex] = 0.0;
        if (LIGHTMAP_MODE == LightmapMode::Directional)
        {
            DecompactTarget[linearIndex] = 0.5;
        }
        return;
    }

    uint rank = countbits(entry.x & ((1u << bit) - 1));
    uint compactIndex = entry.y + rank;

    float3 diffuse = float3(
        CompactedLightmap[compactIndex * 3 + 0],
        CompactedLightmap[compactIndex * 3 + 1],
        CompactedLightmap[compactIndex * 3 + 2]);

    DecompactTarget[linearIndex] = float4(diffuse, 1.0);

    if (LIGHTMAP_MODE == LightmapMode::Directional)
    {
        float4 visibility = CompactedVisibility[compactIndex];
        float3 normalWS = UnpackNormalOctahedron(visibility.w);

        uint compactIndex = compactIndex + Push.compactedCount;
        float luminance = Luminance(diffuse);

        float3 directional = float3(
            CompactedLightmap[compactIndex * 3 + 0],
            CompactedLightmap[compactIndex * 3 + 1],
            CompactedLightmap[compactIndex * 3 + 2]);

        float3 normalizedDirection = normalize(directional / (luminance + 0.0001));

        float alpha = saturate(dot(normalizedDirection, normalWS));

        normalizedDirection.z = -normalizedDirection.z; // unity convention

        DecompactTarget[linearIndex] = mad(float4(normalizedDirection, alpha), 0.5, 0.5);
    }
}
