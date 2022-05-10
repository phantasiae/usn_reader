use crate::raw::usn_journal_wrapper::RawRecords;
use std::mem::size_of;
use windows::Win32::System::Ioctl::{USN_JOURNAL_DATA_V2, USN_RECORD_V2};

pub trait Parser {
    fn parse<R: RawUsnRecord>(self) -> Box<Vec<R>>;
}

impl<const N: usize> Parser for RawRecords<N> {
    fn parse<R: RawUsnRecord>(self) -> Box<Vec<R>> {
        let mut remainder_len = self.len;
        // TODO: why size of i64?
        let mut next = unsafe { self.raw_ptr.as_ptr().offset(size_of::<i64>() as isize) };
        let mut usn_records = Box::new(Vec::new());
        // TODO: comment why lager then 8
        while remainder_len > 8 {
            let usn_record = unsafe { next.cast::<R>().read() };
            let record_len = usn_record.len();
            next = unsafe { next.offset(record_len as _) };
            usn_records.push(usn_record);

            remainder_len -= record_len;
        }

        usn_records
    }
}

pub trait RawUsnRecord {
    fn len(&self) -> u32;
}

impl RawUsnRecord for USN_RECORD_V2 {
    fn len(&self) -> u32 {
        self.RecordLength
    }
}
