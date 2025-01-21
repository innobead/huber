use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kubefire".to_string(),
        source: PackageSource::Github {
            owner: "innobead".to_string(),
            repo: "kubefire".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/kubefire-linux-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["{version}/kubefire-linux-arm64".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
