use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kwctl".to_string(),
        source: PackageSource::Github {
            owner: "kubewarden".to_string(),
            repo: "kwctl".to_string(),
        },

        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["{version}/kwctl-linux-amd64.zip".to_string()],
            ..Default::default()
        })],
        ..Default::default()
    }
}
