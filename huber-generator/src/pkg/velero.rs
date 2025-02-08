use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "velero".to_string(),
        source: PackageSource::Github {
            owner: "vmware-tanzu".to_string(),
            repo: "velero".to_string(),
        },
        ..Default::default()
    }
}
