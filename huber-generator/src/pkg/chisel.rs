use huber_common::model::package::{Package, PackageSource};

use crate::pkg::default_targets;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "chisel".to_string(),
        source: PackageSource::Github {
            owner: "jpillora".to_string(),
            repo: "chisel".to_string(),
        },
        targets: default_targets(),
        ..Default::default()
    }
}
