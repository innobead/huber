use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "cosign".to_string(),
        source: PackageSource::Github {
            owner: "sigstore".to_string(),
            repo: "cosign".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "cosign-linux-amd64".to_string(),
                    "cosigned-linux-amd64".to_string(),
                    "sget-linux-amd64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "cosign-linux-arm64".to_string(),
                    "cosigned-linux-arm64".to_string(),
                    "sget-linux-arm64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "cosign-darwin-amd64".to_string(),
                    "sget-darwin-amd64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec![
                    "cosign-darwin-arm64".to_string(),
                    "sget-darwin-arm64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "cosign-windows-amd64.exe".to_string(),
                    "sget-windows-amd64.exe".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
