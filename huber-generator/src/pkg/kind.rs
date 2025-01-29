use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kind".to_string(),
        source: PackageSource::Github {
            owner: "kubernetes-sigs".to_string(),
            repo: "kind".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["kind-linux-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["kind-linux-arm64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["kind-darwin-amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["kind-windows-amd64".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
