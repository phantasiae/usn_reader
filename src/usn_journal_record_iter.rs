use std::vec::IntoIter;
use crate::usn_journal::UsnJournal;
use crate::usn_journal_record::{RawRecords, Record};

pub struct UsnJournalIter<'a> {
    pub usn_journal: UsnJournal<'a>,
    pub raw_records: IntoIter<Record>,
    pub record: Option<Record>,
    pub start_usn: i64,
}

impl Iterator for UsnJournalIter<'_> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.raw_records.next();
        if next.is_some() {
            self.record = next.clone();
            return next;
        }

        let queried = self.usn_journal.query_data_v2().ok()?;
        let raw_records = self.usn_journal.read::<1_048_576>(self.start_usn, queried.UsnJournalID).ok()?;
        let records = raw_records.parse().ok()?;

        self.raw_records = records.into_iter();
        self.start_usn = raw_records.next;

        self.record.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::usn_journal::UsnJournal;
    use crate::usn_journal_record::Record;
    use crate::usn_journal_record_iter::UsnJournalIter;
    use crate::volume_handle::VolumeHandle;

    #[test]
    #[ignore]
    fn it_should_be_got_next() {
        let vol = VolumeHandle::new('c');
        let usn_journal = UsnJournal::new(&vol);
        let q = usn_journal.query_data_v2().unwrap();
        let raw_records = usn_journal.read::<300>(0, q.UsnJournalID).unwrap();
        let records = raw_records.parse().unwrap();
        let mut iter = UsnJournalIter {
            usn_journal,
            raw_records: records.into_iter(),
            record: None,
            start_usn: 0,
        };

        let r1: Record = iter.next().unwrap().into();
        let r2 = iter.next().unwrap();
        let r3 = iter.next().unwrap();

        assert_ne!(r1.usn, r2.usn);
        assert_ne!(r2.usn, r3.usn);
    }
}
