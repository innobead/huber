use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "fission".to_string(),
        source: PackageSource::Github {
            owner: "fission".to_string(),
            repo: "fission".to_string(),
        },
        ..Default::default()
    }
}
