use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "xh".to_string(),
        source: PackageSource::Github {
            owner: "ducaale".to_string(),
            repo: "xh".to_string(),
        },
        ..Default::default()
    }
}
