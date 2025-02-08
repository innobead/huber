use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "packer".to_string(),
        source: PackageSource::Github {
            owner: "hashicorp".to_string(),
            repo: "packer".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://releases.hashicorp.com/packer/{version}/packer_{version}_linux_amd64.zip"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "https://releases.hashicorp.com/packer/{version}/packer_{version}_linux_arm64.zip"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://releases.hashicorp.com/packer/{version}/packer_{version}_darwin_amd64.zip"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://releases.hashicorp.com/packer/{version}/packer_{version}_windows_amd64.zip"
                        .to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
