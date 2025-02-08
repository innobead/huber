use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "coreutils".to_string(),
        source: PackageSource::Github {
            owner: "uutils".to_string(),
            repo: "coreutils".to_string(),
        },
        ..Default::default()
    }
}
