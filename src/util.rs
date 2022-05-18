use windows::Win32::System::SystemInformation::{GetVersionExW, OSVERSIONINFOW};

pub enum WindowsVersion {
    Other,
    Win7,
    GreaterWin7,
}

pub fn get_os_version() -> WindowsVersion {
    let version = unsafe {
        let mut osvi = OSVERSIONINFOW {
            dwOSVersionInfoSize: std::mem::size_of::<OSVERSIONINFOW>() as u32,
            ..Default::default()
        };
        GetVersionExW(&mut osvi);
        (osvi.dwMajorVersion, osvi.dwMinorVersion)
    };

    match version {
        (6, 1) => WindowsVersion::Win7,
        (6, _) => WindowsVersion::GreaterWin7,
        (10, _) => WindowsVersion::GreaterWin7,
        (11, _) => WindowsVersion::GreaterWin7,
        _ => WindowsVersion::Other,
    }
}
