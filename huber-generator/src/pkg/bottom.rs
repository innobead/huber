use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "bottom".to_string(),
        source: PackageSource::Github {
            owner: "ClementTsang".to_string(),
            repo: "bottom".to_string(),
        },
        ..Default::default()
    }
}
