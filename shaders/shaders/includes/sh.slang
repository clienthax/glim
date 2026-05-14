static const float SH_L0_NORM = float(1.0 / 2.0 * sqrt(1.0 / float.getPi()));
static const float SH_L1_NORM = float(1.0 / 2.0 * sqrt(3.0 / float.getPi()));

struct SHProbe
{
    float3 L0;
    uint pad0;

    float3 L1x;
    float positionX;

    float3 L1y;
    float positionY;

    float3 L1z;
    float positionZ;
};
