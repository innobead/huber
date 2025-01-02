use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kube-bench".to_string(),
        source: PackageSource::Github {
            owner: "aquasecurity".to_string(),
            repo: "kube-bench".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["kube-bench_{version}_linux_amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["kube-bench_{version}_linux_arm64.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
