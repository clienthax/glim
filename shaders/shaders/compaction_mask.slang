#include "includes/core.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint width;
    uint height;
    uint pad0;
    uint pad1;
};

[shader("compute")]
    [numthreads(64, 1, 1)] void
    main(uint3 threadID : SV_DispatchThreadID)
{
    uint2 resolution = uint2(Push.width, Push.height);
    uint index = threadID.x;

    if (index >= (resolution.x * resolution.y))
    {
        return;
    }

    uint2 coords;
    coords.x = threadID.x % resolution.x;
    coords.y = threadID.x / resolution.x;

    float4 visibility = VisibilityBuffer[coords];

    bool visible = visibility.a > 0.0;

    if (!visible)
    {
        return;
    }

    uint bitIndex = index & 31;
    uint bufferIndex = index >> 5;
    uint mask = 1u << bitIndex;
    InterlockedOr(CompactionMaskBuffer[bufferIndex], mask);
}
