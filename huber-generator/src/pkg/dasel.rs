use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "dasel".to_string(),
        source: PackageSource::Github {
            owner: "TomWright".to_string(),
            repo: "dasel".to_string(),
        },
        ..Default::default()
    }
}
