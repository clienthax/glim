# Todo

## Priority
- [ ] Include OIDN dlls

## Features
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
- [ ] Bake sky reflection probe
- [ ] Per chart UV Packing
- [ ] Light Probes/Volumes are noisy with direct light from small emissives
  - [ ] Maybe denoise light probes or do MIS

## Optimization
- [ ] Manually build the LightingData asset
- [ ] Make seam detection faster
- [ ] Make emissive triangle detection check only emissive meshes

## Bugs
- [ ] Sync scene view fov
- [ ] Emissive triangles only detect opaque meshes
- [ ] Backface GI and Transparent flags are set for entire renderer instead of per submesh
- [ ] Can only bake one currently loaded scene
- [ ] Preview crashes when closing the window on linux
- [ ] Preview window doesnt work on KDE Wayland (Fedora) in certain cases
- [ ] Bake reflection probes button starts the built-in baker if the lighting is not baked which could cause confusion
- [ ] Fill blank texture space with lower mip levels like unity does
- [ ] Double sided global illumination doesnt work?
