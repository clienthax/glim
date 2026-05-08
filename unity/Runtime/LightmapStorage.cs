#if UNITY_EDITOR
using System.Collections.Generic;
using System.Linq;
using UnityEditor;
using UnityEditor.SceneManagement;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace stilb
{
    [CreateAssetMenu]
    public class LightmapStorage : ScriptableObject
    {
        [System.Serializable]
        public struct RendererInfo
        {
            public string id;
            public uint lightmapIndex;
            public Vector4 lightmapScaleOffset;
        }

        [System.Serializable]
        public struct LightsInfo
        {
            public string id;
            public bool isBaked;
        }

        [System.Serializable]
        public struct LightmapData
        {
            public Texture2D diffuse;
            public Texture2D directional;
            public Texture2D shadowmask;
        }

        public Scene scene;
        public List<LightmapData> lightmapDatas = new();
        public LightmapsMode lightmapsMode = LightmapsMode.NonDirectional;
        public List<RendererInfo> renderers = new();
        public List<LightsInfo> lights = new();

        public void ApplyLightmaps()
        {
            var lmDatas = new UnityEngine.LightmapData[lightmapDatas.Count];

            for (int i = 0; i < lightmapDatas.Count; i++)
            {
                LightmapData d = lightmapDatas[i];

                lmDatas[i] = new UnityEngine.LightmapData()
                {
                    lightmapColor = d.diffuse,
                    lightmapDir = d.directional,
                    shadowMask = d.shadowmask,
                };
            }

            LightmapSettings.lightmaps = lmDatas.ToArray();
            LightmapSettings.lightmapsMode = lightmapsMode;


            var rendererObjects = new MeshRenderer[renderers.Count];
            var ids = renderers.Select(x =>
            {
                if (GlobalObjectId.TryParse(x.id, out var parsed))
                {
                    return parsed;
                }
                return new GlobalObjectId();
            }).ToArray();
            GlobalObjectId.GlobalObjectIdentifiersToObjectsSlow(ids, rendererObjects);

            for (int i = 0; i < renderers.Count; i++)
            {
                RendererInfo info = renderers[i];

                var mr = rendererObjects[i];
                if (mr == null) continue;

                mr.lightmapIndex = (int)info.lightmapIndex;
                mr.lightmapScaleOffset = info.lightmapScaleOffset;
                EditorUtility.SetDirty(mr);
            }

            var lightObjects = new Light[lights.Count];
            var lightIds = lights.Select(x =>
            {
                if (GlobalObjectId.TryParse(x.id, out var parsed))
                {
                    return parsed;
                }
                return new GlobalObjectId();
            }).ToArray();

            GlobalObjectId.GlobalObjectIdentifiersToObjectsSlow(lightIds, lightObjects);

            for (int i = 0; i < lights.Count; i++)
            {
                var info = lights[i];

                var l = lightObjects[i];
                if (l == null) continue;

                var bakeOutput = new LightBakingOutput
                {
                    isBaked = info.isBaked,
                    lightmapBakeType = LightmapBakeType.Baked,
                    mixedLightingMode = MixedLightingMode.IndirectOnly
                };

                l.bakingOutput = bakeOutput;
                EditorUtility.SetDirty(l);
            }

            EditorSceneManager.MarkSceneDirty(scene);
        }
    }
}
#endif