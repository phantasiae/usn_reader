use anyhow::Result;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_READONLY, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, OPEN_EXISTING,
};

pub struct VolumeHandle {
    pub volume: String,
}

impl VolumeHandle {
    pub fn new(volume: char) -> Self {
        let volume = format!(r#"\\.\{}:"#, volume);

        Self { volume }
    }

    pub fn build_handle(&self) -> Result<HANDLE> {
        unsafe {
            CreateFileW(
                self.volume.clone(),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                std::ptr::null(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_READONLY,
                HANDLE::default(),
            ).map_err(anyhow::Error::from)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::volume_handle::VolumeHandle;

    #[test]
    fn it_should_get_a_error() {
        let h = VolumeHandle::new('2');
        assert!(h.build_handle().is_err());
    }

    #[test]
    #[ignore]
    fn it_should_return_a_handle() {
        let h = VolumeHandle::new('c');
        assert!(h.build_handle().is_err());
    }
}
