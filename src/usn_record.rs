use crate::reader::RecordFetcher;
use crate::usn_journal_record_iter::UsnJournalIter;

#[derive(Clone, Debug, Default)]
pub struct Record {
    pub usn: i64,
    pub timestamp: i64,
}

impl Record {
    pub fn get_unix_timestamp(&self) -> i64 {
        (self.timestamp / 10_000_000) - 11644473600
    }
}

pub struct Records<'a, F: RecordFetcher> {
    pub content: Box<Vec<Record>>,
    pub fetcher: &'a F,
}

impl<'a, F> IntoIterator for Records<'a, F>
where
    F: RecordFetcher,
{
    type Item = Record;
    type IntoIter = UsnJournalIter<'a, F>;

    fn into_iter(self) -> Self::IntoIter {
        UsnJournalIter::new(self.content.into_iter(), self.fetcher)
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::RecordFetcher;
    use crate::usn_record::{Record, Records};
    use anyhow::Result;

    struct MockFetcher {}

    impl RecordFetcher for MockFetcher {
        fn do_fetch(&self) -> Result<Box<Vec<Record>>> {
            Ok(Box::from(vec![
                Record {
                    usn: 1,
                    ..Default::default()
                },
                Record {
                    usn: 2,
                    ..Default::default()
                },
            ]))
        }
    }

    #[test]
    fn it_should_into_iter() {
        let records = Records {
            content: Box::new(vec![
                Record {
                    usn: 1,
                    ..Default::default()
                },
                Record {
                    usn: 2,
                    ..Default::default()
                },
            ]),
            fetcher: &MockFetcher {},
        };

        let mut iter = records.content.into_iter();
        let r1 = iter.next().unwrap();

        assert_eq!(r1.usn, 1);
    }

    #[test]
    fn it_should_convert_timestamp() {
        let record = Record {
            timestamp: 132989000930000000,
            ..Default::default()
        };

        let unix_timestapm = record.get_unix_timestamp();
        assert_eq!(unix_timestapm, 1654426493);
    }
}
