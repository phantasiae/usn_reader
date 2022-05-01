use anyhow::Result;
use std::mem::size_of;
use std::path::PathBuf;
use widestring::U16Str;
pub use windows::Win32::System::Ioctl::{USN_RECORD_UNION, USN_RECORD_V2, USN_RECORD_V3};
use windows::Win32::System::Ioctl::USN_RECORD_V4;

pub struct RawRecords<const N: usize> {
    pub raw_ptr: Box<[u8; N]>,
    pub len: u32,
}

pub struct Record {
    name: String,
    path: Option<PathBuf>,
}

pub trait WithRecordLength {
    fn get_record_length(&self) -> u32;
    fn parse_name(&self) -> String;
}

impl WithRecordLength for USN_RECORD_UNION {
    fn get_record_length(&self) -> u32 {
        unsafe { self.Header.RecordLength }
    }

    fn parse_name(&self) -> String {
        "".to_string()
    }
}

impl WithRecordLength for USN_RECORD_V2 {
    fn get_record_length(&self) -> u32 {
        self.RecordLength
    }

    fn parse_name(&self) -> String {
        let len = (self.FileNameLength as usize) / (size_of::<u16>() * 2);
        unsafe { U16Str::from_ptr(self.FileName.as_ptr(), len) }
            .to_string_lossy()

    }
}

impl WithRecordLength for USN_RECORD_V3 {
    fn get_record_length(&self) -> u32 {
        self.RecordLength
    }

    fn parse_name(&self) -> String {
        "".to_string()
    }
}

impl WithRecordLength for USN_RECORD_V4 {
    fn get_record_length(&self) -> u32 {
        self.Header.RecordLength
    }

    fn parse_name(&self) -> String {
        "".to_string()
    }
}

impl<const N: usize> RawRecords<N> {
    pub unsafe fn raw_parse<T: WithRecordLength>(&self) -> Box<Vec<T>> {
        let mut remainder_len = self.len;

        // TODO: why size of i64?
        let mut next = self.raw_ptr.as_ptr().offset(size_of::<i64>() as isize);
        let mut usn_records = Box::new(Vec::new());

        // TODO: comment why lager then 8
        while remainder_len > 8 {
            let usn_record = next.cast::<T>().read();
            let record_len = usn_record.get_record_length();
            next = next.offset(record_len as _);
            usn_records.push(usn_record);

            remainder_len -= record_len;
        }

        usn_records
    }

    pub fn parse(&self) -> Result<Box<Vec<Record>>> {
        let raw_records = unsafe { self.raw_parse::<USN_RECORD_UNION>() };
        let records = raw_records.iter().map(|r| {
            Record {
                name: unsafe { r.V2.parse_name() },
                path: None
            }
        }).collect();

        Ok(Box::new(records))
    }
}


#[cfg(test)]
mod tests {
    use windows::Win32::System::Ioctl::USN_RECORD_UNION;
    use crate::usn_journal_record::RawRecords;

    #[test]
    fn it_should_be_parsed_with_raw_ujr() {
        let raw_record = RawRecords {
            raw_ptr: Box::new([144, 0, 128, 144, 0, 0, 0, 0, 144, 0, 0, 0, 2, 0, 0, 0, 76, 119, 0, 0, 0, 0, 4, 0, 195, 162, 3, 0, 0, 0, 2, 0, 0, 0, 128, 144, 0, 0, 0, 0, 10, 27, 185, 192, 46, 86, 216, 1, 3, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 80, 0, 60, 0, 51, 0, 51, 0, 48, 0, 66, 0, 67, 0, 50, 0, 51, 0, 53, 0, 68, 0, 66, 0, 55, 0, 65, 0, 55, 0, 56, 0, 56, 0, 50, 0, 52, 0, 52, 0, 67, 0, 57, 0, 68, 0, 67, 0, 66, 0, 65, 0, 52, 0, 68, 0, 50, 0, 56, 0, 68, 0, 65, 0, 51, 0, 57, 0, 70, 0, 53, 0, 56, 0, 66, 0, 52, 0, 48, 0, 56, 0, 53, 0]),
            len: 152,
        };

        let records = unsafe { raw_record.raw_parse() };
        let first_record: &USN_RECORD_UNION = records.first().unwrap();
        let record_length = unsafe { first_record.Header.RecordLength };
        assert_ne!(record_length, 152 + record_length);
        assert_eq!(records.len(), 1);
    }

    #[test]
    fn it_should_be_parsed_with_custom_ujr() {
        let raw_record = RawRecords {
            raw_ptr: Box::new([0, 1, 128, 145, 0, 0, 0, 0, 128, 0, 0, 0, 2, 0, 0, 0, 91, 122, 0, 0, 0, 0, 10, 0, 2, 23, 0, 0, 0, 0, 17, 0, 0, 0, 128, 145, 0, 0, 0, 0, 9, 60, 69, 37, 174, 87, 216, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 66, 0, 60, 0, 109, 0, 111, 0, 122, 0, 112, 0, 108, 0, 117, 0, 103, 0, 105, 0, 110, 0, 45, 0, 98, 0, 108, 0, 111, 0, 99, 0, 107, 0, 45, 0, 100, 0, 105, 0, 103, 0, 101, 0, 115, 0, 116, 0, 50, 0, 53, 0, 54, 0, 46, 0, 115, 0, 98, 0, 115, 0, 116, 0, 111, 0, 114, 0, 101, 0, 0, 0, 128, 0, 0, 0, 2, 0, 0, 0, 91, 122, 0, 0, 0, 0, 10, 0, 2, 23, 0, 0, 0, 0, 17, 0, 128, 0, 128, 145, 0, 0, 0, 0, 9, 60, 69, 37, 174, 87, 216, 1, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 66, 0, 60, 0, 109, 0, 111, 0, 122, 0, 112, 0, 108, 0, 117, 0, 103, 0, 105, 0, 110, 0, 45, 0, 98, 0, 108, 0, 111, 0, 99, 0, 107, 0, 45, 0, 100, 0, 105, 0, 103, 0, 101, 0, 115, 0, 116, 0, 50, 0, 53, 0, 54, 0, 46, 0, 115, 0, 98, 0, 115, 0, 116, 0, 111, 0, 114, 0, 101, 0]),
            len: 264,
        };

        let records = raw_record.parse().unwrap();
        let first_record = records.first().unwrap();
        let second_record = records.last().unwrap();
        assert_eq!(records.len(), 2);
    }
}