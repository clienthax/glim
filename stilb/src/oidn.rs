use std::ffi::{CStr, c_void};

#[repr(C)]
pub struct OIDNDeviceImpl(c_void);
#[repr(C)]
pub struct OIDNFilterImpl(c_void);

pub type OIDNDevice = *mut OIDNDeviceImpl;
pub type OIDNFilter = *mut OIDNFilterImpl;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum OIDNDeviceType {
    Default = 0,
    CPU = 1,
    SYCL = 2,
    CUDA = 3,
    HIP = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum OIDNFormat {
    Undefined = 0,
    Float = 1,
    Float2 = 2,
    Float3 = 3,
    Float4 = 4,
    Half = 257,
    Half2 = 258,
    Half3 = 259,
    Half4 = 260,
}

#[link(name = "OpenImageDenoise")]
#[allow(non_snake_case)]
#[allow(dead_code)]
unsafe extern "C" {
    pub fn oidnNewDevice(device_type: OIDNDeviceType) -> OIDNDevice;
    pub fn oidnCommitDevice(device: OIDNDevice);
    pub fn oidnReleaseDevice(device: OIDNDevice);

    pub fn oidnNewFilter(device: OIDNDevice, filter_type: *const i8) -> OIDNFilter;
    pub fn oidnCommitFilter(filter: OIDNFilter);
    pub fn oidnExecuteFilter(filter: OIDNFilter);
    pub fn oidnReleaseFilter(filter: OIDNFilter);

    pub fn oidnSetSharedFilterImage(
        filter: OIDNFilter,
        name: *const i8,
        ptr: *mut c_void,
        format: OIDNFormat,
        width: usize,
        height: usize,
        byte_offset: usize,
        byte_pixel_stride: usize,
        byte_row_stride: usize,
    );
}

pub fn oidn_denoise(pixels: &[f32], width: usize, height: usize) -> Vec<f32> {
    let pixel_stride = 4 * std::mem::size_of::<f32>();

    let mut output = vec![0.0f32; pixels.len()];

    let device = unsafe { oidnNewDevice(OIDNDeviceType::Default) };
    unsafe { oidnCommitDevice(device) };

    const FILTER_NAME: &CStr = c"RT";
    let filter = unsafe { oidnNewFilter(device, FILTER_NAME.as_ptr()) };

    const COLOR_NAME: &CStr = c"color";
    const OUTPUT_NAME: &CStr = c"output";
    let src_ptr = pixels.as_ptr() as *mut c_void;
    let dst_ptr = output.as_mut_ptr() as *mut c_void;

    unsafe {
        oidnSetSharedFilterImage(
            filter,
            COLOR_NAME.as_ptr(),
            src_ptr,
            OIDNFormat::Float3,
            width,
            height,
            0,
            pixel_stride,
            0,
        )
    };

    unsafe {
        oidnSetSharedFilterImage(
            filter,
            OUTPUT_NAME.as_ptr(),
            dst_ptr,
            OIDNFormat::Float3,
            width,
            height,
            0,
            pixel_stride,
            0,
        )
    };

    unsafe {
        oidnCommitFilter(filter);
        oidnExecuteFilter(filter);

        oidnReleaseFilter(filter);
        oidnReleaseDevice(device);
    }

    output
}
