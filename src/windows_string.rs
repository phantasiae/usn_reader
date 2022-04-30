use widestring::{U16CStr, U16String};
use windows::core::{PCWSTR, PWSTR};

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for PCWSTR {
    fn to_string(&self) -> String {
        let u16cstr = unsafe { U16CStr::from_ptr_str(self.0) };
        u16cstr.to_string_lossy()
    }
}

impl ToString for PWSTR {
    fn to_string(&self) -> String {
        let u16cstr = unsafe { U16CStr::from_ptr_str(self.0) };
        u16cstr.to_string_lossy()
    }
}

#[cfg(test)]
mod tests {
    use windows::core::{IntoParam, Param, PCWSTR, PWSTR};
    use crate::windows_string::ToString;

    #[test]
    fn it_should_converted_correct() {
        let u16cs = &[97, 98, 99, 0];
        let pws = PCWSTR(u16cs.as_ptr());
        let s = pws.to_string();
        assert_eq!(s.len(), 3);
        assert_eq!(s, "abc");
    }
}
