use std::ffi::OsStr;

pub trait OsStrExt {
    fn to_str_direct(&self) -> &str;
    fn to_string_direct(&self) -> String;
}

impl OsStrExt for OsStr {
    fn to_str_direct(&self) -> &str {
        self.to_str().unwrap()
    }

    fn to_string_direct(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}