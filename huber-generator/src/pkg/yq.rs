use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "yq".to_string(),
        source: PackageSource::Github {
            owner: "mikefarah".to_string(),
            repo: "yq".to_string(),
        },
        ..Default::default()
    }
}
