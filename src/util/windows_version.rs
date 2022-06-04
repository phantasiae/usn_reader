use windows::Win32::System::SystemInformation::{GetVersionExW, OSVERSIONINFOW};

pub trait SettingWindowsVersion<'a> {
    fn set_windows_version(&mut self, version: &'a WindowsVersion) {}
}

pub enum WindowsVersion {
    Other,
    Win7,
    GreaterWin7,
    Unknown,
}

impl WindowsVersion {
    pub fn get() -> WindowsVersion {
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
}

#[cfg(test)]
mod tests {
    use crate::util::windows_version::WindowsVersion;

    #[ignore]
    #[test]
    fn it_should_get_os_version() {
        let version = WindowsVersion::get();
    }
}
