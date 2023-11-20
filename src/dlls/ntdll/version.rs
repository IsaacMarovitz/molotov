use crate::dlls::ntdll::ntstatus::NTSTATUS;
use crate::dlls::ntdll::osversioninfo::{RTL_OSVERSIONINFOEXW};
use std::mem;

const CURRENT_VERSION: RTL_OSVERSIONINFOEXW = RTL_OSVERSIONINFOEXW {
    dwOSVersionInfoSize: mem::size_of::<RTL_OSVERSIONINFOEXW>() as u32,
    dwMajorVersion: 10,
    dwMinorVersion: 0,
    dwBuildNumber: 0,
    dwPlatformId: 0,
    szCSDVersion: [0; 128],
    wServicePackMajor: 0,
    wServicePackMinor: 0,
    wSuiteMask: 0,
    wProductType: 0,
    wReserved: 0,
};

#[allow(non_snake_case)]
pub fn RtlGetVersion(info: *mut RTL_OSVERSIONINFOEXW) -> NTSTATUS {
    unsafe {
        let info = &mut *info;
        info.dwMajorVersion = CURRENT_VERSION.dwMajorVersion;
        info.dwMinorVersion = CURRENT_VERSION.dwMinorVersion;
        info.dwPlatformId = CURRENT_VERSION.dwPlatformId;
        info.szCSDVersion = CURRENT_VERSION.szCSDVersion;
        if info.dwOSVersionInfoSize == mem::size_of::<RTL_OSVERSIONINFOEXW>() as u32 {
            info.wServicePackMajor = CURRENT_VERSION.wServicePackMajor;
            info.wServicePackMinor = CURRENT_VERSION.wServicePackMinor;
            info.wSuiteMask = CURRENT_VERSION.wSuiteMask;
            info.wProductType = CURRENT_VERSION.wProductType;
        }
    }

    NTSTATUS::STATUS_SUCCESS
}