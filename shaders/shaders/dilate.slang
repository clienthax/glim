[[vk::push_constant]]
PushConstants Push;
struct PushConstants
{
    uint2 resolution;
    uint pad0;
    uint pad1;
};

[[vk::binding(0)]] RWStructuredBuffer<float4> Tex;

#define TILE 8
#define RADIUS 4
#define HALO (TILE + 2 * RADIUS)

groupshared float4 Cache[HALO * HALO];

uint IndexFromCoords(uint2 coords)
{
    return coords.y * Push.resolution.x + coords.x;
}

int2 ClampToRes(int2 c)
{
    return clamp(c, int2(0, 0), int2(Push.resolution) - 1);
}

[shader("compute")]
    [numthreads(TILE, TILE, 1)] void
    main(uint3 groupID : SV_GroupID, uint3 localID : SV_GroupThreadID, uint3 threadID : SV_DispatchThreadID)
{
    int2 tileOrigin = int2(groupID.xy) * TILE - RADIUS;

    uint threadsPerGroup = TILE * TILE;
    uint cacheCells = HALO * HALO;
    for (uint i = localID.y * TILE + localID.x; i < cacheCells; i += threadsPerGroup)
    {
        int2 local = int2(i % HALO, i / HALO);
        int2 global = ClampToRes(tileOrigin + local);
        Cache[i] = Tex[IndexFromCoords(uint2(global))];
    }
    GroupMemoryBarrierWithGroupSync();

    uint2 coords = threadID.xy;
    if (coords.x >= Push.resolution.x || coords.y >= Push.resolution.y)
    {
        return;
    }

    int2 localCoord = int2(localID.xy) + RADIUS;
    float4 center = Cache[localCoord.y * HALO + localCoord.x];
    if (center.a > 0.0)
    {
        return;
    }

    static const int2 OFFSETS[8] = {
        int2(-1, -1),
        int2(0, -1),
        int2(1, -1),
        int2(-1, 0),
        int2(1, 0),
        int2(-1, 1),
        int2(0, 1),
        int2(1, 1),
    };

    [loop] for (int step = 1; step <= RADIUS; step++)
    {
        [unroll] for (int j = 0; j < 8; j++)
        {
            int2 sampleLocal = localCoord + OFFSETS[j] * step;
            if (sampleLocal.x < 0 || sampleLocal.y < 0 || sampleLocal.x >= HALO || sampleLocal.y >= HALO)
            {
                continue;
            }
            float4 s = Cache[sampleLocal.y * HALO + sampleLocal.x];
            if (s.a > 0.0)
            {
                Tex[IndexFromCoords(coords)] = float4(s.rgb, 1.0);
                return;
            }
        }
    }
}
