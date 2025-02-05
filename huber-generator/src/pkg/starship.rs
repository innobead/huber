use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "starship".to_string(),
        source: PackageSource::Github {
            owner: "starship".to_string(),
            repo: "starship".to_string(),
        },
        ..Default::default()
    }
}
