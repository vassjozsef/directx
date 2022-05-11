use winapi::ctypes::c_void;
use winapi::shared::dxgi::{IDXGIDevice, IID_IDXGIDevice};
use winapi::um::d3d11::{
    D3D11CreateDevice, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_DEBUG,
    D3D11_SDK_VERSION,
};
use winapi::um::d3dcommon::{
    D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1,
    D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_9_1, D3D_FEATURE_LEVEL_9_2,
    D3D_FEATURE_LEVEL_9_3,
};

use windows::core::Abi;
use windows::Win32::Graphics::Dxgi::IDXGIDevice as IDXGIDevice_windows;

const DIRECT3D_FEATURE_LEVELS: &[u32] = &[
    D3D_FEATURE_LEVEL_11_1,
    D3D_FEATURE_LEVEL_11_0,
    D3D_FEATURE_LEVEL_10_1,
    D3D_FEATURE_LEVEL_10_0,
    D3D_FEATURE_LEVEL_9_3,
    D3D_FEATURE_LEVEL_9_2,
    D3D_FEATURE_LEVEL_9_1,
];

fn main() {
    let levels = DIRECT3D_FEATURE_LEVELS;
    let flags = match cfg!(debug) {
        true => D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_DEBUG,
        false => D3D11_CREATE_DEVICE_BGRA_SUPPORT,
    };
    let driver_type = D3D_DRIVER_TYPE_HARDWARE;

    let mut device = std::ptr::null_mut();
    let mut context = std::ptr::null_mut();

    let hr = unsafe {
        D3D11CreateDevice(
            std::ptr::null_mut(),
            driver_type,
            std::ptr::null_mut(),
            flags,
            levels.as_ptr(),
            levels.len() as u32,
            D3D11_SDK_VERSION,
            &mut device,
            std::ptr::null_mut(),
            &mut context,
        )
    };

    dbg!(hr);
    dbg!(device);

    let device = unsafe { device.as_ref().unwrap() };

    let mut p: *mut c_void = std::ptr::null_mut();
    let hr = unsafe { device.QueryInterface(&IID_IDXGIDevice, &mut p) };

    dbg!(hr);
    dbg!(p);

    let dxgi_device = unsafe { (p as *mut IDXGIDevice).as_ref().unwrap() };
    let rc = unsafe { dxgi_device.AddRef() };
    dbg!(rc);

    let mut priority = -1;
    let hr = unsafe { dxgi_device.GetGPUThreadPriority(&mut priority) };

    dbg!(hr);
    dbg!(priority);

    // convert to windows crate IDXGIDevice
    let dxgi_device2 = unsafe { IDXGIDevice_windows::from_abi(p as *mut _).unwrap() };
    dbg!(&dxgi_device2);
    let priority = unsafe { dxgi_device2.GetGPUThreadPriority() }.ok().unwrap();
    dbg!(priority);

    // convert back to winapi IDXGIDevice
    let dxgi_device3_ptr: *mut IDXGIDevice = unsafe { std::mem::transmute(dxgi_device2) };
    dbg!(dxgi_device3_ptr);
    let dxgi_device3 = unsafe { &*dxgi_device3_ptr };
    let mut priority = -1;
    let hr = unsafe { dxgi_device3.GetGPUThreadPriority(&mut priority) };
    dbg!(hr);
    dbg!(priority);
}
