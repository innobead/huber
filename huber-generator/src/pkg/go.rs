use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "go".to_string(),
        source: PackageSource::Github {
            owner: "golang".to_string(),
            repo: "go".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://golang.org/dl/go{version}.linux-amd64.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "https://golang.org/dl/go{version}.linux-arm64.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://golang.org/dl/go{version}.darwin-amd64.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec![
                    "https://golang.org/dl/go{version}.darwin-arm64.tar.gz".to_string()
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://golang.org/dl/go{version}.windows-amd64.zip".to_string()
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
