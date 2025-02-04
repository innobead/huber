use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "flux2".to_string(),
        source: PackageSource::Github {
            owner: "fluxcd".to_string(),
            repo: "flux2".to_string(),
        },
        ..Default::default()
    }
}
