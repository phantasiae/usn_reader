use crate::raw::usn_journal_wrapper::UsnJournalWrapper;
use crate::reader::RecordFetcher;
use crate::usn_record::Record;
use std::vec::IntoIter;

pub struct UsnJournalIter<'a, F: RecordFetcher> {
    pub fetcher: &'a F,
    pub records: IntoIter<Record>,
    pub current: usize,
}

impl<'a, F> UsnJournalIter<'a, F>
where
    F: RecordFetcher,
{
    pub fn new(records: IntoIter<Record>, fetcher: &'a F) -> Self {
        Self {
            fetcher,
            records,
            current: 0,
        }
    }
}

impl<'a, F> Iterator for UsnJournalIter<'a, F>
where
    F: RecordFetcher,
{
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let mut records = self.records.clone();

        let next = records.nth(self.current);
        if next.is_some() {
            self.current += 1;
            return next;
        }

        self.current = 0;
        let next_block = self.fetcher.do_fetch().ok()?;
        let current_record = next_block.into_iter();

        self.records = current_record;
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::RecordFetcher;
    use crate::usn_journal_record_iter::UsnJournalIter;
    use crate::usn_record::{Record, Records};
    use anyhow::Result;

    struct MockFetcher {}

    impl RecordFetcher for MockFetcher {
        fn do_fetch(&self) -> Result<Box<Vec<Record>>> {
            Ok(Box::from(vec![
                Record {
                    usn: 3,
                    ..Default::default()
                },
                Record {
                    usn: 4,
                    ..Default::default()
                },
            ]))
        }
    }

    #[test]
    fn it_should_be_got_none() {
        let mut iter = UsnJournalIter::new(vec![].into_iter(), &MockFetcher {});

        let first = iter.next();
        assert!(first.is_none());
    }

    #[test]
    fn it_should_be_got_next() {
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
        let mut iter = records.into_iter();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();
        let forth = iter.next().unwrap();

        assert_eq!(first.usn, 1);
        assert_eq!(second.usn, 2);
        assert_eq!(third.usn, 3);
        assert_eq!(forth.usn, 4);
    }
}
