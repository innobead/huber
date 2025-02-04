use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "delta".to_string(),
        source: PackageSource::Github {
            owner: "dandavison".to_string(),
            repo: "delta".to_string(),
        },
        ..Default::default()
    }
}
