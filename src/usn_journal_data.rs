use crate::raw::usn_journal_wrapper::UsnJournalWrapper;
use anyhow::Result;
use windows::Win32::System::Ioctl::{
    USN_JOURNAL_DATA_V0, USN_JOURNAL_DATA_V1, USN_JOURNAL_DATA_V2,
};

pub struct UsnJournalData<'a, U: UsnJournalWrapper, D: Default> {
    usn_journal: &'a U,
    pub raw: D,
}

pub struct UsnJournalDataFactory<'a, U>
where
    U: UsnJournalWrapper,
{
    usn_journal: &'a U,
}

impl<'a, U> UsnJournalDataFactory<'a, U>
where
    U: UsnJournalWrapper,
{
    pub fn new(usn_journal: &'a U) -> Self {
        Self { usn_journal }
    }

    pub fn query(&self) -> Result<UsnJournalData<U, USN_JOURNAL_DATA_V2>> {
        let data: USN_JOURNAL_DATA_V2 =
            unsafe { self.usn_journal.raw_query::<USN_JOURNAL_DATA_V2>()? };
        Ok(UsnJournalData {
            usn_journal: &self.usn_journal,
            raw: data,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::usn_journal_wrapper::{RawRecords, UsnJournalWrapper};
    use crate::usn_journal_data::UsnJournalDataFactory;
    use anyhow::Result;
    use windows::Win32::System::Ioctl::USN_JOURNAL_DATA_V2;

    struct TestUsnJournal {}

    impl UsnJournalWrapper for TestUsnJournal {
        unsafe fn raw_create(&self) {
            unreachable!()
        }
        unsafe fn raw_query<D: Default>(&self) -> Result<D> {
            Ok(D::default())
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
        assert_eq!(data.raw, USN_JOURNAL_DATA_V2::default());
    }
}
