#include "includes/core.slang"

[[vk::push_constant]]
PushConstants Push;

struct PushConstants
{
    uint2 resolution;
    uint groupIndex;
    uint pad0;

    uint2 rtResolution;
    uint2 pad1;
};

struct Varyings
{
    float4 positionCS : SV_Position;
    nointerpolation uint primitiveID : TEXCOORD2;
};

[shader("vertex")] Varyings vertex(uint vertexID : SV_VulkanVertexID)
{
    Varyings o;

    uint primitiveID = vertexID / 3;
    uint index = vertexID % 3;

    uint vIndex = Indices[primitiveID * 3 + index];
    Vertex v = Vertices[vIndex];

    if (v.GetLightmapGroup() != Push.groupIndex)
    {
        o.positionCS = asfloat(-1);
        o.primitiveID = uint::maxValue;
    }
    else
    {
        float2 resolution = Push.resolution;
        float2 rtResolution = Push.rtResolution;

        float2 scale = resolution / rtResolution;
        float2 uv = v.GetUV() / resolution;

        uv.y = 1.0 - uv.y;
        uv *= scale;

        o.positionCS = float4(uv * 2.0 - 1.0, 0.0, 1.0);
        o.primitiveID = primitiveID;
    }

    return o;
}

    [shader("fragment")] uint fragment(Varyings varyings)
    : SV_Target
{
    return varyings.primitiveID;
}
