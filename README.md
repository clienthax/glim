# Todo

## Priority
- [ ] Alpha test
- [ ] Spot lights
- [ ] Spot light cookie with default unity cookie
- [ ] UV Packing with scale offset
- [ ] Adjust sample positions
- [ ] Directional lightmaps

## Other
- [ ] Figure out why light probes are a bit darker
- [ ] Include OIDN dlls
- [ ] Area lights
- [ ] Seam stitching
- [ ] Add support for CWBVH
- [ ] UV Packing per chart
- [ ] Deringing
- [ ] SH Lightmaps
- [ ] Light tree
- [ ] Blue noise
- [ ] Stop closing and opening the scene twice
- [ ] Proper sync for bake loop
- [ ] Probe occlusion
- [ ] Try to stop unity from slowing down the bake start for no reason

## Easy
- [ ] Better panic handling
- [ ] Sync scene view fov
- [ ] Return codes for bake success, fail, cancel
- [ ] Log callback
- [ ] Double sided global illumination

# Complete
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
