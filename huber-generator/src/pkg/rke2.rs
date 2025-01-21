use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "rke2".to_string(),
        source: PackageSource::Github {
            owner: "rancher".to_string(),
            repo: "rke2".to_string(),
        },

        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["{version}/rke2.linux-amd64".to_string()],
            ..Default::default()
        })],
        ..Default::default()
    }
}
