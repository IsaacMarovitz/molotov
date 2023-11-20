use crate::dlls::ntdll::ntstatus::NTSTATUS;
use crate::dlls::ntdll::osversioninfo::{RTL_OSVERSIONINFOEXW};
use std::mem;

#[allow(non_snake_case)]
pub fn RtlGetVersion(info: *mut RTL_OSVERSIONINFOEXW) -> NTSTATUS {
    unsafe {
        let info = &mut *info;
        info.dwMajorVersion = 0;
        info.dwMinorVersion = 0;
        info.dwPlatformId = 0;
        if info.dwOSVersionInfoSize == mem::size_of::<RTL_OSVERSIONINFOEXW>() as u32 {
            info.wServicePackMajor = 0;
            info.wServicePackMinor = 0;
            info.wSuiteMask = 0;
            info.wProductType = 0;
        }
    }

    NTSTATUS::STATUS_SUCCESS
}