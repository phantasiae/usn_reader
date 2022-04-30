use std::path::Path;
use anyhow::{anyhow, Result};
use windows::core::IntoParam;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Storage::FileSystem::{CreateFileW, FILE_ATTRIBUTE_READONLY, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING};

pub struct VolumeHandle {
    pub volume: String,
    pub handle: HANDLE,
}

impl From<&Path> for VolumeHandle {
    fn from(p: &Path) -> Self {
        let volume = format!(r#"\\.\{}:"#, p.);

        let handle = unsafe {
            CreateFileW(
                volume.clone(),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                std::ptr::null(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_READONLY,
                HANDLE::default(),
            )
        };
        VolumeHandle {
            volume,
            handle,
        }
    }
}

impl VolumeHandle {
    pub fn new(volume: char) -> Self {
        let volume = format!(r#"\\.\{}:"#, volume);
        let handle = unsafe {
            CreateFileW(
                volume.clone(),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                std::ptr::null(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_READONLY,
                HANDLE::default(),
            )
        };

        Self { volume, handle }
    }

    pub fn get_handle(&self) -> Result<HANDLE> {
        self.handle
            .ok()
            .map_err(|e| { anyhow!("get handle {} error: {}.", self.volume, e) })
    }

    #[cfg(why)]
    pub unsafe fn get_std_handle(&self) -> Result<std::os::windows::raw::HANDLE> {
        self.handle.ok()
            .map(|h| { h.0 as _ })
            .map_err(|e| { anyhow!("get {} handle error: {}.", self.volume, e) })
    }
}

#[cfg(test)]
mod tests {
    use crate::volume_handle::VolumeHandle;

    #[test]
    fn it_should_get_a_error() {
        let h = VolumeHandle::new('2');
        assert!(h.get_handle().is_err());
    }

    #[test]
    #[ignore]
    fn it_should_return_a_handle() {
        let h = VolumeHandle::new('c');
        assert!(h.get_handle().is_err());
    }
}
