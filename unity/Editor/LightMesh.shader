Shader "Unlit/Light Mesh"
{
    Properties
    {
        [Enum(Point, 0, Spot, 1, Directional, 2, Area, 4)] _LightType ("Light Type", Int) = 0
        _LightColor ("Light Color", Color) = (1,1,1,1)
        _LightIntensity ("Light Intensity", Float) = 1.0
        _LightRadius ("Light Radius", Float) = 0.1
    }
    SubShader
    {
        Tags { "RenderType"="Opaque" }
        Cull Front

        Pass
        {
            HLSLPROGRAM
            #pragma vertex vert
            #pragma fragment frag
            #pragma target 5.0

            #include "UnityCG.cginc"

            float _LightRadius;
            float4 _LightColor;
            float _LightIntensity;
            uint _LightType;

            struct Attributes
            {
                float3 positionOS : POSITION;
                float2 uv0 : TEXCOORD0;
            };

            struct Varyings
            {
                float4 positionCS : SV_POSITION;
                float3 positionWS : POSITIONWS;
            };

            float3 CameraPositionWS()
            {
                return UNITY_MATRIX_I_V._m03_m13_m23;
            }

            Varyings vert(Attributes attributes)
            {
                Varyings o;

                float3 positionWS = mul(UNITY_MATRIX_M, float4(attributes.positionOS, 1.0)).xyz;
                o.positionCS = mul(UNITY_MATRIX_VP, float4(positionWS, 1.0));

                o.positionWS = positionWS;

                return o;
            }

            float SphereIntersect(float3 ro, float3 rd, float4 sph)
            {
                float3 oc = ro - sph.xyz;
                float b = dot( oc, rd );
                float c = dot( oc, oc ) - sph.w*sph.w;
                float h = b*b - c;
                if( h<0.0 ) return -1.0;
                h = sqrt( h );
                return -b - h;
            }

            struct FragOutput
            {
                float4 color : SV_Target;
                float depth : SV_DepthGreaterEqual;
            };

            FragOutput frag(Varyings varyings)
            {
                float3 objectPosition = UNITY_MATRIX_M._m03_m13_m23;

                float objectScale = float3(
                    length(UNITY_MATRIX_M._m00_m10_m20),
                    length(UNITY_MATRIX_M._m01_m11_m21),
                    length(UNITY_MATRIX_M._m02_m12_m22)
                );

                float3 ro = CameraPositionWS();
                float3 rd = -normalize(ro - varyings.positionWS);

                float t = SphereIntersect(ro, rd, float4(objectPosition, _LightRadius));

                if (t <= -1.0)
                {
                    discard;
                }

                float4 color = 1.0;
                color.rgb = _LightColor.rgb * _LightIntensity * 3.14159;

                float3 positionWS = ro + rd * t;

                float4 positionCS = mul(UNITY_MATRIX_VP, float4(positionWS, 1.0));
                float ndcDepth = positionCS.z / positionCS.w;
                #if defined(SHADER_API_GLCORE) || defined(SHADER_API_OPENGL) || defined(SHADER_API_GLES) || defined(SHADER_API_GLES3)
                    ndcDepth = ndcDepth * 0.5 + 0.5;
                #endif

                FragOutput o;
                o.color = color;
                o.depth = ndcDepth;
                return o;
            }

            ENDHLSL
        }
    }
}
