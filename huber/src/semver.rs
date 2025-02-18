use std::cmp::Ordering;
use std::str::FromStr;

use regex::Regex;
use semver::Version;

pub trait VersionCompareTrait {
    fn cmp_version(&self, version: &str) -> Option<Ordering>;
}

impl VersionCompareTrait for String {
    fn cmp_version(&self, version: &str) -> Option<Ordering> {
        let regex = Regex::new(r"^v").unwrap();

        let self_version = Version::from_str(&regex.replace(self, ""));
        let comparing_version = Version::from_str(&regex.replace(version, ""));

        if self_version.is_err() || comparing_version.is_err() {
            return Some(Ordering::Equal);
        }

        self_version
            .unwrap()
            .partial_cmp(&comparing_version.unwrap())
    }
}
