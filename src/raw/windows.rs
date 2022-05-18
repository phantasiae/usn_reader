use crate::raw::usn_journal_wrapper::{RawRecords, UsnJournalWrapper};
use crate::raw::volume_handle::VolumeHandle;
use anyhow::{anyhow, Result};
use std::any::Any;
use std::mem::{size_of_val, transmute};
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::Ioctl::{
    FSCTL_QUERY_USN_JOURNAL, FSCTL_READ_USN_JOURNAL, READ_USN_JOURNAL_DATA_V0, USN_JOURNAL_DATA_V0,
    USN_JOURNAL_DATA_V1, USN_JOURNAL_DATA_V2,
};
use windows::Win32::System::IO::DeviceIoControl;

pub struct WindowsUsnJournal<'a> {
    pub handle: &'a VolumeHandle,
}

impl<'a> WindowsUsnJournal<'a> {
    pub fn new(volume: &'a VolumeHandle) -> Self {
        Self { handle: volume }
    }
}

impl<'a> UsnJournalWrapper for WindowsUsnJournal<'a> {
    unsafe fn raw_create(&self) {
        todo!()
    }

    unsafe fn raw_query<D: RawUsnJournalData + Default>(&self) -> Result<D> {
        let mut result = D::default();
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
        .as_bool()
        {
            return Err(anyhow!(GetLastError().0));
        }

        Ok(result)
    }

    unsafe fn raw_read<const N: usize>(
        &self,
        start_usn: i64,
        usn_journal_id: u64,
    ) -> Result<RawRecords<N>, anyhow::Error> {
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
            true => Ok(RawRecords {
                raw_ptr: output,
                len: ret_bytes,
            }),
            false => Ok(RawRecords {
                raw_ptr: output,
                len: 0,
            }),
        }
    }

    unsafe fn raw_enum<const N: usize>(&self) -> Result<RawRecords<N>, anyhow::Error> {
        todo!()
    }

    unsafe fn raw_delete(&self) {
        todo!()
    }
}

pub trait RawUsnJournalData {}

impl RawUsnJournalData for USN_JOURNAL_DATA_V0 {}
impl RawUsnJournalData for USN_JOURNAL_DATA_V1 {}
impl RawUsnJournalData for USN_JOURNAL_DATA_V2 {}
