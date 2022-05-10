pub mod raw;
pub mod reader;
pub mod usn_journal_data;
pub mod usn_journal_record;
pub mod usn_journal_record_iter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
