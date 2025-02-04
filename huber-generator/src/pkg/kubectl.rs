use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kubectl".to_string(),
        source: PackageSource::Github {
            owner: "kubernetes".to_string(),
            repo: "kubernetes".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://dl.k8s.io/release/v{version}/bin/linux/amd64/kubectl".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "https://dl.k8s.io/release/v{version}/bin/linux/arm64/kubectl".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "https://dl.k8s.io/release/v{version}/bin/linux/arm/kubectl".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://dl.k8s.io/release/v{version}/bin/darwin/amd64/kubectl".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec![
                    "https://dl.k8s.io/release/v{version}/bin/darwin/arm64/kubectl".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://dl.k8s.io/release/v{version}/bin/windows/amd64/kubectl.exe"
                        .to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
