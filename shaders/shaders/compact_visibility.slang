#include "includes/core.slang"
#include "includes/util.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint2 resolution;
    uint offset;
    uint pad1;
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

        uint primitiveID = VisibilityBuffer[coords];
        uint2 uv = coords;
        uv.y = Push.resolution.y - uv.y;
        uint2 visibility = uint2(primitiveID, PackCoords(uv));

        CompactedVisibility[compactIndex] = visibility;
    }
}
