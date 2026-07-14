#include "includes/core.slang"
#include "includes/util.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint2 resolution;
    uint offset;
    uint pad0;
};

[numthreads(8, 8, 1)] void
main(uint3 threadID : SV_DispatchThreadID)
{
    uint2 coords = threadID.xy;

    if (coords.x >= Push.resolution.x || coords.y >= Push.resolution.y)
    {
        return;
    }

    uint index = InterleaveWithZero(coords.x) |
                 (InterleaveWithZero(coords.y) << 1);

    uint word = index >> 5; // /32
    uint bit = index & 31;  // %32

    uint2 entry = CompactionBuffer[Push.offset + word];

    bool active = (entry.x & (1u << bit)) != 0;

    if (active)
    {
        uint rank = countbits(entry.x & ((1u << bit) - 1));
        uint compactIndex = entry.y + rank;

        uint2 expandedVisibility = VisibilityBuffer[coords];
        uint primitiveID = expandedVisibility.x;
        uint2 originalCoords = UnpackCoords(expandedVisibility.y);
        float2 uv = float2(originalCoords) + 0.5;
        float2 bary = BaryFromUV(primitiveID, uv);

        float3 visibility = float3(bary, asfloat(primitiveID));
        CompactedVisibility[compactIndex * 3 + 0] = visibility.x;
        CompactedVisibility[compactIndex * 3 + 1] = visibility.y;
        CompactedVisibility[compactIndex * 3 + 2] = visibility.z;
    }
}
