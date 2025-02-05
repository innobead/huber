use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "zoxide".to_string(),
        source: PackageSource::Github {
            owner: "ajeetdsouza".to_string(),
            repo: "zoxide".to_string(),
        },
        ..Default::default()
    }
}
