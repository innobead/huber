use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "sonobuoy".to_string(),
        source: PackageSource::Github {
            owner: "vmware-tanzu".to_string(),
            repo: "sonobuoy".to_string(),
        },
        ..Default::default()
    }
}
