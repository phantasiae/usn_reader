use crate::raw::usn_journal_wrapper::UsnJournalWrapper;
use crate::util::windows_version::{MatchVersion, WindowsVersion};
use anyhow::Result;
use windows::Win32::System::Ioctl::{
    USN_JOURNAL_DATA_V0, USN_JOURNAL_DATA_V1, USN_JOURNAL_DATA_V2,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Data {
    pub usn_journal_id: u64,
    pub first_usn: i64,
    pub next_usn: i64,
    pub lowest_valid_usn: i64,
    pub max_usn: i64,
    pub maximum_size: u64,
    pub allocation_delta: u64,
    pub min_supported_major_version: Option<u16>,
    pub max_supported_major_version: Option<u16>,
    pub flags: Option<u32>,
    pub range_track_chunk_size: Option<u64>,
    pub range_track_file_size_threshold: Option<i64>,
}

impl From<USN_JOURNAL_DATA_V0> for Data {
    fn from(u: USN_JOURNAL_DATA_V0) -> Self {
        Self {
            usn_journal_id: u.UsnJournalID,
            first_usn: u.FirstUsn,
            next_usn: u.NextUsn,
            lowest_valid_usn: u.LowestValidUsn,
            max_usn: u.MaxUsn,
            maximum_size: u.MaximumSize,
            allocation_delta: u.AllocationDelta,
            min_supported_major_version: None,
            max_supported_major_version: None,
            flags: None,
            range_track_chunk_size: None,
            range_track_file_size_threshold: None,
        }
    }
}

impl From<USN_JOURNAL_DATA_V1> for Data {
    fn from(u: USN_JOURNAL_DATA_V1) -> Self {
        Self {
            usn_journal_id: u.UsnJournalID,
            first_usn: u.FirstUsn,
            next_usn: u.NextUsn,
            lowest_valid_usn: u.LowestValidUsn,
            max_usn: u.MaxUsn,
            maximum_size: u.MaximumSize,
            allocation_delta: u.AllocationDelta,
            min_supported_major_version: Some(u.MinSupportedMajorVersion),
            max_supported_major_version: Some(u.MaxSupportedMajorVersion),
            flags: None,
            range_track_chunk_size: None,
            range_track_file_size_threshold: None,
        }
    }
}

impl From<USN_JOURNAL_DATA_V2> for Data {
    fn from(u: USN_JOURNAL_DATA_V2) -> Self {
        Self {
            usn_journal_id: u.UsnJournalID,
            first_usn: u.FirstUsn,
            next_usn: u.NextUsn,
            lowest_valid_usn: u.LowestValidUsn,
            max_usn: u.MaxUsn,
            maximum_size: u.MaximumSize,
            allocation_delta: u.AllocationDelta,
            min_supported_major_version: Some(u.MinSupportedMajorVersion),
            max_supported_major_version: Some(u.MaxSupportedMajorVersion),
            flags: Some(u.Flags),
            range_track_chunk_size: Some(u.RangeTrackChunkSize),
            range_track_file_size_threshold: Some(u.RangeTrackFileSizeThreshold),
        }
    }
}

#[derive(Debug)]
pub enum DataVer {
    V0,
    V1,
    V2,
}

pub struct UsnJournalData<'a, U: UsnJournalWrapper> {
    usn_journal: &'a U,
    pub ver: DataVer,
    pub data: Data,
}

pub struct UsnJournalDataFactory<'a, U>
where
    U: UsnJournalWrapper,
{
    usn_journal: &'a U,
    version: &'a WindowsVersion,
}

impl<'a, U> MatchVersion<'a> for UsnJournalDataFactory<'a, U>
where
    U: UsnJournalWrapper,
{
    fn by_win_version(&mut self, version: &'a WindowsVersion) {
        self.version = &version;
    }
}

impl<'a, U> UsnJournalDataFactory<'a, U>
where
    U: UsnJournalWrapper,
{
    pub fn new(usn_journal: &'a U) -> Self {
        Self {
            usn_journal,
            version: &WindowsVersion::Unknown,
        }
    }

    pub fn query(&self) -> Result<UsnJournalData<'a, U>> {
        let result = match &self.version {
            WindowsVersion::GreaterWin7 => UsnJournalData {
                usn_journal: self.usn_journal,
                ver: DataVer::V2,
                data: unsafe { self.usn_journal.raw_query::<USN_JOURNAL_DATA_V2>()?.into() },
            },
            WindowsVersion::Win7 => UsnJournalData {
                usn_journal: self.usn_journal,
                ver: DataVer::V1,
                data: unsafe { self.usn_journal.raw_query::<USN_JOURNAL_DATA_V1>()?.into() },
            },
            _ => UsnJournalData {
                usn_journal: self.usn_journal,
                ver: DataVer::V0,
                data: unsafe { self.usn_journal.raw_query::<USN_JOURNAL_DATA_V0>()?.into() },
            },
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::usn_journal_wrapper::{RawRecords, UsnJournalWrapper};
    use crate::raw::windows::RawUsnJournalData;
    use crate::usn_journal_data::UsnJournalDataFactory;
    use crate::util::windows_version::{MatchVersion, WindowsVersion};
    use anyhow::Result;
    use windows::Win32::System::Ioctl::USN_JOURNAL_DATA_V2;

    struct TestUsnJournal {}
    trait Mock {
        fn get_usn_journal_id(&self) -> u64;
    }
    impl Mock for USN_JOURNAL_DATA_V2 {
        fn get_usn_journal_id(&self) -> u64 {
            self.UsnJournalID
        }
    }

    impl UsnJournalWrapper for TestUsnJournal {
        unsafe fn raw_create(&self) {
            unreachable!()
        }
        unsafe fn raw_query<D: RawUsnJournalData + Default>(&self) -> Result<D> {
            Ok(Default::default())
        }
        unsafe fn raw_read<const N: usize>(&self, _: i64, _: u64) -> Result<RawRecords<N>> {
            unreachable!()
        }
        unsafe fn raw_enum<const N: usize>(&self) -> Result<RawRecords<N>> {
            unreachable!()
        }
        unsafe fn raw_delete(&self) {
            unreachable!()
        }
    }

    #[test]
    fn it_has_available_to_query() {
        let factory = UsnJournalDataFactory::new(&TestUsnJournal {});
        let data = factory.query().unwrap();

        assert_eq!(data.data.usn_journal_id, 0);
    }

    #[test]
    fn it_has_available_to_specify_os_query() {
        let mut factory = UsnJournalDataFactory::new(&TestUsnJournal {});
        let version = WindowsVersion::get();
        factory.by_win_version(&version);
        let data = factory.query().unwrap();

        assert_eq!(data.data.usn_journal_id, 0);
    }
}
