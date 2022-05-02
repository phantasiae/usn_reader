use anyhow::Result;

pub struct RawRecords<const N: usize> {
    pub raw_ptr: Box<[u8; N]>,
    pub len: u32,
}

pub trait UsnJournalWrapper {
    unsafe fn raw_create(&self);
    unsafe fn raw_query<D: Default>(&self) -> Result<D>;
    unsafe fn raw_read<const N: usize>(&self, start_usn: i64, usn_journal_id: u64) -> Result<RawRecords<N>>;
    unsafe fn raw_enum<const N: usize>(&self) -> Result<RawRecords<N>>;
    unsafe fn raw_delete(&self);
}
