use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "croc".to_string(),
        source: PackageSource::Github {
            owner: "schollz".to_string(),
            repo: "croc".to_string(),
        },
        ..Default::default()
    }
}
