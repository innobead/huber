use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ollama".to_string(),
        source: PackageSource::Github {
            owner: "ollama".to_string(),
            repo: "ollama".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["ollama-linux-amd64.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["ollama-linux-arm64.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["ollama-darwin".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["ollama-windows-amd64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
