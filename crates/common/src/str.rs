use std::cmp::Ordering;
use std::ffi::OsStr;
use std::str::FromStr;

use semver::Version;

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

pub trait VersionCompareTrait {
    fn cmp_version(&self, version: &str) -> Option<Ordering>;
}

impl VersionCompareTrait for String {
    fn cmp_version(&self, version: &str) -> Option<Ordering> {
        let msg = format!("{} should be semantic version", version);
        Version::from_str(&self)
            .expect(&msg)
            .partial_cmp(&Version::from_str(version).expect(&msg))
    }
}
