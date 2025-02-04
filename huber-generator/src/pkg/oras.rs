use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "oras".to_string(),
        source: PackageSource::Github {
            owner: "oras-project".to_string(),
            repo: "oras".to_string(),
        },
        ..Default::default()
    }
}
