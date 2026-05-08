using System.IO;
using UnityEditor;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEditor.SceneManagement;
using Microsoft.SqlServer.Server;

namespace stilb
{
    public class LightingData
    {
        private const string TempScenePath = "Packages/io.github.z3y.stilb/Editor/Scene/Temp.unity";
        private const string TempLightingDataPath = "Packages/io.github.z3y.stilb/Editor/Scene/Temp/LightingData.asset";

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
            throw new System.Exception("a");
            return lightingDataAsset;
        }
    }
}