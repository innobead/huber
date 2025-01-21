use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "submariner".to_string(),
        source: PackageSource::Github {
            owner: "submariner-io".to_string(),
            repo: "submariner-operator".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["subctl-v{version}-linux-amd64.tar.xz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["subctl-v{version}-linux-arm64.tar.xz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["subctl-v{version}-darwin-amd64.tar.xz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["subctl-v{version}-windows-amd64.exe.tar.xz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
