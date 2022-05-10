use crate::raw::parser::Parser;
use crate::raw::usn_journal_wrapper::{RawRecords, UsnJournalWrapper};
use anyhow::{anyhow, Result};
pub use windows::Win32::System::Ioctl::{USN_RECORD_UNION, USN_RECORD_V2, USN_RECORD_V3};

pub struct UsnRecordFactory<'a, U>
where
    U: UsnJournalWrapper,
{
    usn_journal: &'a U,
    pub start_usn: i64,
    pub usn_journal_id: Option<u64>,
}

impl<'a, U: UsnJournalWrapper> UsnRecordFactory<'a, U> {
    pub fn new(usn_journal: &'a U) -> Self {
        Self {
            usn_journal,
            start_usn: 0,
            usn_journal_id: None,
        }
    }

    pub fn set_usn_journal_id(&mut self, id: u64) -> &Self {
        self.usn_journal_id = Some(id);
        self
    }

    pub fn read<const N: usize>(&self) -> Result<UsnJournalRecord<U, N>> {
        let usn_journal_id = self
            .usn_journal_id
            .ok_or(anyhow!("usn journal id not found."))?;

        let raw_records = unsafe { self.usn_journal.raw_read(self.start_usn, usn_journal_id)? };
        Ok(UsnJournalRecord {
            usn_journal: &self.usn_journal,
            raw: raw_records,
            next: None,
        })
    }
}

pub struct UsnJournalRecord<'a, U: UsnJournalWrapper, const N: usize> {
    pub usn_journal: &'a U,
    pub raw: RawRecords<N>,
    pub next: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct Record {
    pub usn: i64,
}

impl<'a, U: UsnJournalWrapper, const N: usize> UsnJournalRecord<'a, U, N> {
    pub fn parse(self) -> Box<Vec<Record>> {
        // TODO: how represent in here each windows version.
        let raw_records = self.raw.parse::<USN_RECORD_V2>();
        Box::new(raw_records.iter().map(|r| Record { usn: r.Usn }).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::usn_journal_wrapper::UsnJournalWrapper;
    use crate::usn_journal_record::{RawRecords, UsnRecordFactory};
    use anyhow::Result;

    struct TestUsnJournal {}

    impl UsnJournalWrapper for TestUsnJournal {
        unsafe fn raw_create(&self) {
            unreachable!()
        }

        unsafe fn raw_query<D: Default>(&self) -> Result<D> {
            unreachable!()
        }

        unsafe fn raw_read<const N: usize>(&self, _: i64, _: u64) -> Result<RawRecords<N>> {
            let p = [
                144u8, 0, 128, 144, 0, 0, 0, 0, 144, 0, 0, 0, 2, 0, 0, 0, 76, 119, 0, 0, 0, 0, 4,
                0, 195, 162, 3, 0, 0, 0, 2, 0, 0, 0, 128, 144, 0, 0, 0, 0, 10, 27, 185, 192, 46,
                86, 216, 1, 3, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 80, 0, 60, 0, 51, 0,
                51, 0, 48, 0, 66, 0, 67, 0, 50, 0, 51, 0, 53, 0, 68, 0, 66, 0, 55, 0, 65, 0, 55, 0,
                56, 0, 56, 0, 50, 0, 52, 0, 52, 0, 67, 0, 57, 0, 68, 0, 67, 0, 66, 0, 65, 0, 52, 0,
                68, 0, 50, 0, 56, 0, 68, 0, 65, 0, 51, 0, 57, 0, 70, 0, 53, 0, 56, 0, 66, 0, 52, 0,
                48, 0, 56, 0, 53, 0,
            ];
            let mut pp = [0u8; N];
            pp.copy_from_slice(&p);
            Ok(RawRecords {
                raw_ptr: Box::new(pp),
                len: 152,
            })
        }

        unsafe fn raw_enum<const N: usize>(&self) -> Result<RawRecords<N>> {
            unreachable!()
        }

        unsafe fn raw_delete(&self) {
            unreachable!()
        }
    }

    struct TestUsnJournal2 {}

    impl UsnJournalWrapper for TestUsnJournal2 {
        unsafe fn raw_create(&self) {
            unreachable!()
        }

        unsafe fn raw_query<D: Default>(&self) -> Result<D> {
            unreachable!()
        }

        unsafe fn raw_read<const N: usize>(&self, _: i64, _: u64) -> Result<RawRecords<N>> {
            let p = [
                0, 1, 128, 145, 0, 0, 0, 0, 128, 0, 0, 0, 2, 0, 0, 0, 91, 122, 0, 0, 0, 0, 10, 0,
                2, 23, 0, 0, 0, 0, 17, 0, 0, 0, 128, 145, 0, 0, 0, 0, 9, 60, 69, 37, 174, 87, 216,
                1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 66, 0, 60, 0, 109, 0, 111, 0,
                122, 0, 112, 0, 108, 0, 117, 0, 103, 0, 105, 0, 110, 0, 45, 0, 98, 0, 108, 0, 111,
                0, 99, 0, 107, 0, 45, 0, 100, 0, 105, 0, 103, 0, 101, 0, 115, 0, 116, 0, 50, 0, 53,
                0, 54, 0, 46, 0, 115, 0, 98, 0, 115, 0, 116, 0, 111, 0, 114, 0, 101, 0, 0, 0, 128,
                0, 0, 0, 2, 0, 0, 0, 91, 122, 0, 0, 0, 0, 10, 0, 2, 23, 0, 0, 0, 0, 17, 0, 128, 0,
                128, 145, 0, 0, 0, 0, 9, 60, 69, 37, 174, 87, 216, 1, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 32, 0, 0, 0, 66, 0, 60, 0, 109, 0, 111, 0, 122, 0, 112, 0, 108, 0, 117, 0,
                103, 0, 105, 0, 110, 0, 45, 0, 98, 0, 108, 0, 111, 0, 99, 0, 107, 0, 45, 0, 100, 0,
                105, 0, 103, 0, 101, 0, 115, 0, 116, 0, 50, 0, 53, 0, 54, 0, 46, 0, 115, 0, 98, 0,
                115, 0, 116, 0, 111, 0, 114, 0, 101, 0,
            ];
            let mut pp = [0u8; N];
            pp.copy_from_slice(&p);
            Ok(RawRecords {
                raw_ptr: Box::new(pp),
                len: 264,
            })
        }

        unsafe fn raw_enum<const N: usize>(&self) -> Result<RawRecords<N>> {
            unreachable!()
        }

        unsafe fn raw_delete(&self) {
            unreachable!()
        }
    }

    #[test]
    fn it_should_has_one_record() {
        let mut factory = UsnRecordFactory::new(&TestUsnJournal {});
        factory.set_usn_journal_id(0);
        let raw_usn_records = factory.read::<148>().unwrap();
        let usn_records = raw_usn_records.parse();
        let first = usn_records.first().unwrap();

        assert_eq!(usn_records.len(), 1);
        assert_eq!(first.usn, 2424307712);
    }

    #[test]
    fn it_should_has_two_records() {
        let mut factory = UsnRecordFactory::new(&TestUsnJournal2 {});
        factory.set_usn_journal_id(0);
        let raw_usn_records = factory.read::<262>().unwrap();
        let usn_records = raw_usn_records.parse();
        let first = usn_records.first().unwrap();
        let second = usn_records.get(1).unwrap();

        assert_eq!(usn_records.len(), 2);
        assert_eq!(first.usn, 2441084928);
        assert_eq!(second.usn, 2441085056);
    }
}
