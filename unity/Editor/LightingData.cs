using System.IO;
using UnityEditor;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEditor.SceneManagement;

namespace stilb
{
    public class LightingData
    {
        private const string TempScenePath = "Packages/io.github.z3y.stilb/Editor/Scene/Temp.unity";
        private const string TempLightingDataPath = "Packages/io.github.z3y.stilb/Editor/Scene/Temp/LightingData.asset";

        // todo https://github.com/pema99/GITweaks/blob/master/Editor/GITweaksLightingDataAssetEditor.cs

        public static System.Reflection.PropertyInfo InspectorModeObject =
                    typeof(SerializedObject).GetProperty("inspectorMode", System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Instance);

        public struct SerializedObjectID : System.IEquatable<SerializedObjectID>
        {
            public long MainLFID; // If prefab, LFID in MeshRenderer in prefab stage, else LFID of object
            public long PrefabLFID; // If prefab, LFID of "Prefab instance" object, points to prefab

            public SerializedObjectID(long main, long prefab)
            {
                MainLFID = main;
                PrefabLFID = prefab;
            }

            public bool Equals(SerializedObjectID other) => other.MainLFID == MainLFID && other.PrefabLFID == PrefabLFID;
            public override bool Equals(object obj) => obj is SerializedObjectID id && Equals(id);
            public static bool operator ==(SerializedObjectID a, SerializedObjectID b) => a.Equals(b);
            public static bool operator !=(SerializedObjectID a, SerializedObjectID b) => !(a == b);
            public override int GetHashCode() => System.HashCode.Combine(MainLFID, PrefabLFID);
        }

        public static SerializedObjectID ObjectToSOI(Object obj)
        {
            using var mainSO = new SerializedObject(obj);
            InspectorModeObject.SetValue(mainSO, InspectorMode.DebugInternal);
            long lfid = mainSO.FindProperty("m_LocalIdentfierInFile").longValue;

            var prefabInstance = mainSO.FindProperty("m_PrefabInstance");
            if (prefabInstance.objectReferenceValue != null)
            {
                using var prefabInstanceSO = new SerializedObject(prefabInstance.objectReferenceValue);
                InspectorModeObject.SetValue(prefabInstanceSO, InspectorMode.DebugInternal);

                using var correspondingSO = new SerializedObject(mainSO.FindProperty("m_CorrespondingSourceObject").objectReferenceValue);
                InspectorModeObject.SetValue(correspondingSO, InspectorMode.DebugInternal);

                long sourceLFID = correspondingSO.FindProperty("m_LocalIdentfierInFile").longValue;
                long prefabLFID = prefabInstanceSO.FindProperty("m_LocalIdentfierInFile").longValue;

                return new SerializedObjectID(sourceLFID, prefabLFID);
            }
            else
            {
                return new SerializedObjectID(lfid, 0);
            }
        }

        public static LightingDataAsset CreateAsset(Scene scene)
        {
            var scenePath = scene.path;

            Scene tempScene = EditorSceneManager.OpenScene(TempScenePath, OpenSceneMode.Additive);

            // foreach (GameObject obj in scene.GetRootGameObjects())
            // {
            //     if (obj.GetComponentsInChildren<Light>(true).Length > 0 ||
            //         obj.GetComponentsInChildren<LightProbeGroup>(true).Length > 0)
            //     {
            //         GameObject copy = Object.Instantiate(obj);
            //         SceneManager.MoveGameObjectToScene(copy, tempScene);
            //     }
            // }


            EditorSceneManager.CloseScene(scene, true);
            EditorSceneManager.SaveScene(tempScene);
            EditorSceneManager.SetActiveScene(tempScene);

            if (!Lightmapping.Bake())
            {
                throw new System.Exception("Bake failed");
            }


            foreach (GameObject obj in tempScene.GetRootGameObjects())
            {
                Object.DestroyImmediate(obj);
            }
            EditorSceneManager.SaveScene(tempScene);

            scene = EditorSceneManager.OpenScene(scenePath, OpenSceneMode.Single);
            EditorSceneManager.SetActiveScene(scene);
            EditorSceneManager.CloseScene(tempScene, true);

            string destPath = Path.Combine(Path.GetDirectoryName(scenePath), "LightingData.asset").Replace("\\", "/");
            AssetDatabase.CopyAsset(TempLightingDataPath, destPath);
            AssetDatabase.ImportAsset(destPath);

            var lightingDataAsset = AssetDatabase.LoadAssetAtPath<LightingDataAsset>(destPath);
            using var lda = new SerializedObject(lightingDataAsset);

            var sceneAsset = AssetDatabase.LoadAssetAtPath<SceneAsset>(scene.path);

            var sceneProp = lda.FindProperty("m_Scene");
            Debug.Assert(sceneProp != null);
            sceneProp.objectReferenceValue = sceneAsset;

            lda.ApplyModifiedPropertiesWithoutUndo();

            Lightmapping.lightingDataAsset = lightingDataAsset;
            return lightingDataAsset;
        }
    }
}