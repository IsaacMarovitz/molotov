use crate::types::{ULONG, USHORT, UCHAR, WCHAR};

#[allow(non_snake_case)]
#[repr(C)]
pub struct OSVERSIONINFOEXW {
    pub dwOSVersionInfoSize: ULONG,
    pub dwMajorVersion: ULONG,
    pub dwMinorVersion: ULONG,
    pub dwBuildNumber: ULONG,
    pub dwPlatformId: ULONG,
    pub szCSDVersion: [WCHAR; 128],
    pub wServicePackMajor: USHORT,
    pub wServicePackMinor: USHORT,
    pub wSuiteMask: USHORT,
    pub wProductType: UCHAR,
    pub wReserved: UCHAR
}

pub(crate) type POSVERSIONINFOEXW = OSVERSIONINFOEXW;
pub(crate) type LPOSVERSIONINFOEXW = OSVERSIONINFOEXW;
#[allow(non_camel_case_types)]
pub(crate) type RTL_OSVERSIONINFOEXW = OSVERSIONINFOEXW;
#[allow(non_camel_case_types)]
pub(crate) type PRTL_OSVERSIONINFOEXW = OSVERSIONINFOEXW;

#[allow(non_snake_case)]
#[repr(C)]
pub struct OSVERSIONINFOW {
    pub dwOSVersionInfoSize: ULONG,
    pub dwMajorVersion: ULONG,
    pub dwMinorVersion: ULONG,
    pub dwBuildNumber: ULONG,
    pub dwPlatformId: ULONG,
    pub szCSDVersion: [WCHAR; 128],
}

pub(crate) type POSVERSIONINFOW = OSVERSIONINFOEXW;
pub(crate) type LPOSVERSIONINFOW = OSVERSIONINFOEXW;
#[allow(non_camel_case_types)]
pub(crate) type RTL_OSVERSIONINFOW = OSVERSIONINFOEXW;
#[allow(non_camel_case_types)]
pub(crate) type PRTL_OSVERSIONINFOW = OSVERSIONINFOEXW;