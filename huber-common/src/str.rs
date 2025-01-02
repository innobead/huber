use std::cmp::Ordering;
use std::ffi::OsStr;
use std::str::FromStr;

use regex::Regex;
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
        let regex = Regex::new(r"^v").unwrap();

        let self_version = Version::from_str(&regex.replace(self, ""))
            .expect(&format!("{} should be semantic version", self));
        let comparing_version = Version::from_str(&regex.replace(version, ""))
            .expect(&format!("{} should be semantic version", version));

        self_version.partial_cmp(&comparing_version)
    }
}
