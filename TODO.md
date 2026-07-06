# Todo

## Priority
- [ ] Some negatively scaled exported objects have flipped normals
- [ ] Include OIDN dlls
- [ ] Adjust sample positions before baking
- [ ] Light Volumes

## Features
- [x] UV Packing
  - [x] Scale offset
  - [ ] Per chart
  - [x] Padding
- [x] Spot lights
- [x] Bake lightprobes with new shader
- [ ] Directional lightmaps
- [ ] Higher resolution alpha
- [ ] Terrain Support
- [ ] Sky light
- [ ] Light Cookies
- [ ] Area lights
- [ ] Light Probes Deringing
- [ ] Shadowmask
- [ ] Subtractive
- [ ] Ambient Occlusion
- [ ] SH Lightmaps
- [ ] Adaptive Probe Volumes
- [ ] Probe occlusion
- [ ] Emissive multiplier
- [ ] Indirect multiplier
- [ ] Add support for CWBVH
- [ ] Global fix seams instead of per renderer
- [ ] Bake sky reflection probe

## Optimization
- [ ] Proper sync for bake loop
- [ ] Try to stop unity from slowing down the bake start for no reason
- [ ] Manually build the LightingData asset
- [ ] Make seam detection faster
- [ ] Make emissive triangle detection check only emissive meshes
- [ ] Create visibility shader only once and reuse
- [ ] Memory optimizations (compress previous diffuse between bounces, destroy emission etc)
- [ ] Deduplicate light probe positions
- [x] Sample alpha in bake init shader as well to skip some rays

## Bugs
- [ ] Sync scene view fov
- [ ] Previous diffuse is flipped on Y
- [ ] handle not optimal swapchain
- [ ] Emissive triangles only detect opaque meshes
- [ ] Backface GI and Transparent flags are set for entire renderer instead of per submesh
- [ ] Can only bake one currently loaded scene
- [ ] Preview crashes when closing on linux
- [ ] Preview window doesnt work on KDE Wayland (Fedora)
- [ ] Bake reflection probes button starts the built-in baker if the lighting is not baked which could cause confusion

# Complete
- [x] Fix URP light falloff
- [x] Preview emission doesnt have 1 bounce
- [x] Match point/spot light shadow radius
- [x] No licence yet
- [x] OpenGL unity is flipped xd
- [x] Bake reflection probes button with super sampling
- [x] Log and progress callback
- [x] Alpha test
- [x] Conservative rasterization
- [x] Return codes for bake success, fail, cancel
- [x] Better panic handling
- [x] Seam stitching
- [x] Figure out why light probes are a bit darker
- [x] Blue noise
- [x] Double sided global illumination
- [x] Clamp max samples and bounces
- [x] Move test to another crate so gltf and image are not dependencies
- [x] Configurable nearest and linear sampler
- [x] Configurable probe samples and bounces
- [X] L2 SH
- [x] Export light probe positions and accumulate SH
- [x] Set all the globals in the unity meta pass
- [x] OIDN2 bindings and apply denoise

## Readme

- Supports only linear color space
