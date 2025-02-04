use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "nerdctl".to_string(),
        source: PackageSource::Github {
            owner: "containerd".to_string(),
            repo: "nerdctl".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["nerdctl-{version}-linux-amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["nerdctl-{version}-linux-arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm(PackageManagement {
                artifact_templates: vec!["nerdctl-{version}-linux-arm-v7.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
