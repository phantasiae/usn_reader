use crate::raw::usn_journal_wrapper::UsnJournalWrapper;
use crate::usn_journal_data::UsnJournalDataFactory;
use crate::usn_journal_record::UsnRecordFactory;
use crate::usn_record::{Record, Records};
use anyhow::Result;

pub trait RecordFetcher {
    fn do_fetch(&self) -> Result<Box<Vec<Record>>>;
}

struct Reader<'a, U: UsnJournalWrapper> {
    pub usn_journal: &'a U,
}

impl<'a, U> Reader<'a, U>
where
    U: UsnJournalWrapper,
{
    pub fn new(usn_journal: &'a U) -> Self {
        Self { usn_journal }
    }

    pub fn read(&self) -> Result<Records<Self>> {
        let records = Records {
            content: self.do_fetch()?,
            fetcher: self,
        };
        Ok(records)
    }
}

impl<'a, U> RecordFetcher for Reader<'a, U>
where
    U: UsnJournalWrapper,
{
    fn do_fetch(&self) -> Result<Box<Vec<Record>>> {
        let data_factory = UsnJournalDataFactory::new(self.usn_journal);
        let data = data_factory.query()?;
        let mut record_factory = UsnRecordFactory::new(self.usn_journal);
        record_factory.set_usn_journal_id(data.data.usn_journal_id);
        let raw_records = record_factory.read::<65535>()?;
        let records = raw_records.parse();
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::usn_journal_wrapper::{RawRecords, UsnJournalWrapper};
    use crate::reader::{Reader, RecordFetcher};
    use anyhow::Result;

    struct TestUsnJournal {}

    impl UsnJournalWrapper for TestUsnJournal {
        unsafe fn raw_create(&self) {
            unreachable!()
        }

        unsafe fn raw_query<D: Default>(&self) -> Result<D> {
            Ok(Default::default())
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
            pp[0..148].clone_from_slice(&p);
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

    #[test]
    fn it_should_be_read() {
        let reader = Reader::new(&TestUsnJournal {});
        let records = reader.do_fetch().unwrap();
        let only_one = records.first().unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(only_one.usn, 2424307712);
    }

    #[test]
    fn it_should_be_read_and_into() {
        let reader = Reader::new(&TestUsnJournal {});
        let records = reader.read().unwrap();
        let mut iter = records.into_iter();
        let only_one = iter.next().unwrap();

        assert_eq!(only_one.usn, 2424307712);
    }
}
