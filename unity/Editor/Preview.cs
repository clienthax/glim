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
    public class Preview
    {
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