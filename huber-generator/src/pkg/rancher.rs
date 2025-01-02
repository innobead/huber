use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "rancher".to_string(),
        source: PackageSource::Github {
            owner: "rancher".to_string(),
            repo: "rancher".to_string(),
        },

        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["rancherd-amd64.tar.gz".to_string()],
            ..Default::default()
        })],
        ..Default::default()
    }
}
