use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "loc".to_string(),
        source: PackageSource::Github {
            owner: "cgag".to_string(),
            repo: "loc".to_string(),
        },
        ..Default::default()
    }
}
