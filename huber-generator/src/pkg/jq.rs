use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "jq".to_string(),
        source: PackageSource::Github {
            owner: "jqlang".to_string(),
            repo: "jq".to_string(),
        },
        ..Default::default()
    }
}
