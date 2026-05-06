using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using UnityEditor;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UIElements;

namespace stilb
{
    public class Preview : EditorWindow
    {
        [MenuItem("Stilb/Preview")]
        public static void Open()
        {
            GetWindow<Preview>("Lightmap Preview");
        }

        Bindings.StilbConfig config;
        Bindings.LightmapSettings previewSettings;


        public void CreateGUI()
        {
            config = new Bindings.StilbConfig
            {
                coordinate_system = Bindings.CoordinateSystem.Unity,
                is_preview = true,
                preview_settings = previewSettings,
                throttle_preview_ms = 10,
            };

            previewSettings = new Bindings.LightmapSettings
            {
                width = 1024,
                height = 1024,
                max_samples = 512,
                bounce_count = 3,
            };

            Button startButton = new Button
            {
                text = "Start Preview",
                style =
                {
                    height = 50
                }
            };
            startButton.clicked += () =>
            {
                var camera = SceneView.lastActiveSceneView.camera;
                config.camera_position = camera.transform.position;
                config.camera_forward = camera.transform.forward;
                config.preview_settings = previewSettings;
                StartPreview(config);
            };
            rootVisualElement.Add(startButton);

            var width = new IntegerField("Width") { value = (int)previewSettings.width };
            width.RegisterValueChangedCallback(evt => previewSettings.width = (uint)evt.newValue);
            rootVisualElement.Add(width);

            var height = new IntegerField("Height") { value = (int)previewSettings.height };
            height.RegisterValueChangedCallback(evt => previewSettings.height = (uint)evt.newValue);
            rootVisualElement.Add(height);

            var maxSamples = new IntegerField("Max Samples") { value = (int)previewSettings.max_samples };
            maxSamples.RegisterValueChangedCallback(evt => previewSettings.max_samples = (uint)evt.newValue);
            rootVisualElement.Add(maxSamples);

            var bounceCount = new IntegerField("Bounces") { value = (int)previewSettings.bounce_count };
            bounceCount.RegisterValueChangedCallback(evt => previewSettings.bounce_count = (uint)evt.newValue);
            rootVisualElement.Add(bounceCount);

            var throttle = new IntegerField("Throttle Preview (ms)") { value = (int)config.throttle_preview_ms };
            throttle.RegisterValueChangedCallback(evt => config.throttle_preview_ms = (uint)evt.newValue);
            rootVisualElement.Add(throttle);
        }

        public static void StartPreview(Bindings.StilbConfig config)
        {
            var ctx = new BakeContext();


            var thread = new Thread(() =>
            {
                try
                {
                    var app = Bindings.app_new(config);


                    for (int i = 0; i < ctx.sceneMesh.Count; i++)
                    {
                        var data = ctx.sceneMesh[i];

                        unsafe
                        {
                            fixed (Vector3* vPtr = data.vertices)
                            fixed (Vector3* nPtr = data.normals)
                            fixed (Vector2* uPtr = data.uvs)
                            fixed (int* iPtr = data.triangles)
                            {
                                var exportedMesh = new Bindings.Mesh
                                {
                                    vertices = vPtr,
                                    normals = nPtr,
                                    uvs = uPtr,
                                    indices = (uint*)iPtr,
                                    vertices_length = (uint)data.vertices.Length,
                                    indices_length = (uint)data.triangles.Length,
                                    lightmap_group = data.groupIndex,
                                };

                                Bindings.app_add_mesh(app, exportedMesh);
                            }
                        }
                    }

                    foreach (var light in ctx.sceneLights)
                    {
                        Bindings.app_add_light(app, light);
                    }

                    foreach (var group in ctx.groups)
                    {
                        unsafe
                        {
                            fixed (Color32* albedoPtr = group.albedo)
                            fixed (Color* emissionsPtr = group.emission)
                            {
                                Bindings.app_add_lightmap_group(
                                    app,
                                    group.settings,
                                    (byte*)albedoPtr,
                                    (uint)(group.albedo.Length * 4),
                                    (float*)emissionsPtr,
                                    (uint)(group.emission.Length * 4)
                                );
                            }
                        }
                    }


                    Bindings.app_run(app);

                    Bindings.app_destroy(app);

                }
                catch (Exception e)
                {
                    Debug.LogException(e);
                }
            });

            thread.SetApartmentState(ApartmentState.STA);
            thread.IsBackground = true;
            thread.Start();


        }

    }
}