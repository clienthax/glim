use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    ptr::null_mut,
    slice,
};

use crate::{
    LightmapGroup, LightmapSettings, Stilb, StilbConfig,
    lights::Light,
    math::Vector3,
    mesh::{FfiMesh, Mesh},
    sh::SHProbe,
    start_bake,
};

#[repr(u32)]
pub enum ErrorCode {
    Success = 0,
    Error = 1,
}

#[unsafe(no_mangle)]
pub extern "C" fn app_new(config: StilbConfig) -> *mut Stilb {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let app = Stilb::new(config.clone());
        Box::into_raw(Box::new(app))
    }));

    match result {
        Ok(val) => val,
        Err(_) => null_mut() as *mut Stilb,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn app_add_mesh(app: *mut Stilb, mesh: FfiMesh) -> ErrorCode {
    if app.is_null() {
        return ErrorCode::Error;
    }
    let result = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };

        Mesh::append_ffi_mesh(
            &mut app.cpu_mesh,
            mesh,
            app.config.coordinate_system,
            &mut app.seams,
            !app.config.is_preview,
        );
    }));

    match result {
        Ok(_) => ErrorCode::Success,
        Err(_) => ErrorCode::Error,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn app_add_light(app: *mut Stilb, mut light: Light) -> ErrorCode {
    if app.is_null() {
        return ErrorCode::Error;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };

        let system = app.config.coordinate_system;
        light.position.transform_space(system);
        light.direction.transform_space(system);

        light.direction = Vector3::ZERO - light.direction;

        // todo:
        light.shadow_radius_or_angle = light.shadow_radius_or_angle.max(0.001);

        app.cpu_lights.push(light);
    }));

    match result {
        Ok(_) => ErrorCode::Success,
        Err(_) => ErrorCode::Error,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn app_add_lightmap_group(
    app: *mut Stilb,
    settings: LightmapSettings,
    albedo_pixels: *const u8,
    albedo_pixels_length: u32,
    emission_pixels: *const f32,
    emission_pixels_length: u32,
) -> ErrorCode {
    if app.is_null() {
        return ErrorCode::Error;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };

        let emission_pixels =
            unsafe { slice::from_raw_parts(emission_pixels, emission_pixels_length as usize) };

        let albedo_pixels =
            unsafe { slice::from_raw_parts(albedo_pixels, albedo_pixels_length as usize) };

        let group = LightmapGroup::new(app, settings, albedo_pixels, emission_pixels);
        app.groups.push(group);
    }));

    match result {
        Ok(_) => ErrorCode::Success,
        Err(_) => ErrorCode::Error,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn app_run(app: *mut Stilb) -> ErrorCode {
    if app.is_null() {
        return ErrorCode::Error;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };
        start_bake(app);
    }));

    match result {
        Ok(_) => ErrorCode::Success,
        Err(_) => ErrorCode::Error,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn app_add_probe(app: *mut Stilb, mut position: Vector3) -> ErrorCode {
    if app.is_null() {
        return ErrorCode::Error;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let app = unsafe { &mut *app };

        let system = app.config.coordinate_system;
        position.transform_space(system);

        let probe = SHProbe {
            position,
            pad0: 0,
            l0: Vector3::ZERO,
            pad1: 0,
            l1_1: Vector3::ZERO,
            pad2: 0,
            l10: Vector3::ZERO,
            pad3: 0,
            l11: Vector3::ZERO,
            pad4: 0,
            l2_2: Vector3::ZERO,
            pad5: 0,
            l2_1: Vector3::ZERO,
            pad6: 0,
            l20: Vector3::ZERO,
            pad7: 0,
            l21: Vector3::ZERO,
            pad8: 0,
            l22: Vector3::ZERO,
            pad9: 0,
        };

        app.probes.push(probe);
    }));

    match result {
        Ok(_) => ErrorCode::Success,
        Err(_) => ErrorCode::Error,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn app_destroy(app: *mut Stilb) -> ErrorCode {
    if app.is_null() {
        return ErrorCode::Error;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        if !app.is_null() {
            // Take ownership back from the pointer and let Box drop it
            let mut _app = unsafe { Box::from_raw(app) };

            println!("App destroyed ");
        }
    }));

    match result {
        Ok(_) => ErrorCode::Success,
        Err(_) => ErrorCode::Error,
    }
}
