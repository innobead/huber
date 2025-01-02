use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "k6".to_string(),
        source: PackageSource::Github {
            owner: "k6io".to_string(),
            repo: "k6".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "k6-v{version}-linux-amd64.tar.gz".to_string(),
                    "k6-v{version}-linux64.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["k6-v{version}-linux-arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "k6-v{version}-macos-amd64.zip".to_string(),
                    "k6-v{version}-mac.zip".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "k6-v{version}-windows-amd64.zip".to_string(),
                    "k6-v{version}-win64.zip".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
