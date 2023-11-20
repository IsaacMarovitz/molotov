use crate::dlls::ntdll::ntstatus::NTSTATUS;
use crate::dlls::ntdll::osversioninfo::RTL_OSVERSIONINFOEXW;

pub fn RtlGetVersion(info: *mut RTL_OSVERSIONINFOEXW) -> NTSTATUS {
    unsafe {
        let info = &mut *info;
        info.dwMajorVersion = 0;
        info.dwMinorVersion = 0;
        info.dwPlatformId = 0;
        if info.dwOSVersionInfoSize == 0 {

        }
    }

    NTSTATUS::STATUS_SUCCESS
}