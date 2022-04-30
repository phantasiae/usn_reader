use anyhow::{anyhow, Result};
use std::mem::{size_of, size_of_val, transmute, transmute_copy};
use windows::Win32::Foundation::{GetLastError, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_READONLY, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::Ioctl::{FSCTL_QUERY_USN_JOURNAL, FSCTL_READ_USN_JOURNAL, READ_USN_JOURNAL_DATA_V0, USN_JOURNAL_DATA_V0, USN_JOURNAL_DATA_V1, USN_JOURNAL_DATA_V2, USN_RECORD_UNION};
use windows::Win32::System::IO::DeviceIoControl;
use crate::usn_journal_record::RawRecords;
use crate::volume_handle::VolumeHandle;

pub struct UsnJournal<'a> {
    pub handle: &'a VolumeHandle
}

impl<'a> UsnJournal<'a> {
    pub fn new(volume: &'a VolumeHandle) -> Self {
        Self {
            handle: volume,
        }
    }

    pub unsafe fn raw_create(&self) {
        todo!()
    }

    pub unsafe fn raw_query_data<UJD: Default>(&self) -> Result<UJD> {
        let mut result = UJD::default();
        let mut ret_bytes = 0;

        if !DeviceIoControl(
            self.handle.get_handle()?,
            FSCTL_QUERY_USN_JOURNAL,
            std::ptr::null(),
            0,
            &mut result as *mut _ as _,
            size_of_val(&result) as _,
            &mut ret_bytes,
            std::ptr::null_mut(),
        )
        .as_bool() {
            return Err(anyhow!(GetLastError().0));
        }

        Ok(result)
    }

    pub fn query_data_v0(&self) -> Result<USN_JOURNAL_DATA_V0> {
        unsafe { self.raw_query_data() }
    }

    pub fn query_data_v1(&self) -> Result<USN_JOURNAL_DATA_V1> {
        unsafe { self.raw_query_data() }
    }

    pub fn query_data_v2(&self) -> Result<USN_JOURNAL_DATA_V2> {
        unsafe { self.raw_query_data() }
    }

    pub unsafe fn raw_read<const N: usize>(
        &self,
        start_usn: i64,
        usn_journal_id: u64,
    ) -> Result<(Box<[u8; N]>, u32)> {
        let mut output = Box::new([0u8; N]);
        let mut ret_bytes = 0;
        let input = READ_USN_JOURNAL_DATA_V0 {
            StartUsn: start_usn,
            ReasonMask: u32::MAX,
            ReturnOnlyOnClose: 0,
            Timeout: 0,
            BytesToWaitFor: 0,
            UsnJournalID: usn_journal_id,
        };

        match DeviceIoControl(
            self.handle.get_handle()?,
            FSCTL_READ_USN_JOURNAL,
            transmute(&input),
            size_of_val(&input) as _,
            transmute(output.as_mut_ptr()),
            output.len() as _,
            &mut ret_bytes,
            std::ptr::null_mut(),
        )
        .as_bool()
        {
            true => Ok((output, ret_bytes)),
            false => Ok((output, 0)),
}
    }

    pub fn read<const N: usize>(&self, start_usn: i64, usn_journal_id: u64) -> Result<RawRecords<N>> {
        let (records, len): (Box<[_;N]>, _) = unsafe { self.raw_read(start_usn, usn_journal_id)? };
        
        Ok(RawRecords {
            raw_ptr: records,
            len,
        })
    }

    pub unsafe fn raw_delete(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use windows::Win32::System::Ioctl::USN_JOURNAL_DATA_V2;
    use crate::volume_handle::VolumeHandle;

    use super::UsnJournal;

    #[test]
    fn it_should_queried_usn() {
        let vh = VolumeHandle::new('c');
        let mut r = UsnJournal::new(&vh);
        let usn = r.query_data_v2().unwrap();
        assert_ne!(usn.UsnJournalID, 0);
    }

    #[test]
    #[ignore]
    fn it_should_read_usn() {
        let vh = VolumeHandle::new('c');
        let mut r = UsnJournal::new(&vh);
        let q = r.query_data_v2().unwrap();
        let records: (Box<[u8; 300]>, _) = unsafe { r.raw_read(0, q.UsnJournalID).unwrap() };
    }

    #[test]
    #[ignore]
    fn it_should_parse_usn_records() {
        let vh = VolumeHandle::new('c');
        let r = UsnJournal::new(&vh);
        let q: USN_JOURNAL_DATA_V2 = r.query_data_v2().unwrap();
        let raw_records: (Box<[_; 65560]>, _) =
            unsafe { r.raw_read(0, q.UsnJournalID).unwrap() };
        assert_ne!(raw_records.1, 0);
    }
}
