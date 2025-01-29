use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kompose".to_string(),
        source: PackageSource::Github {
            owner: "kubernetes".to_string(),
            repo: "kompose".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["kompose-linux-arm64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["kompose-linux-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["kompose-darwin-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["kompose-windows-amd64.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
