use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "chisel".to_string(),
        source: PackageSource::Github {
            owner: "jpillora".to_string(),
            repo: "chisel".to_string(),
        },
        ..Default::default()
    }
}
