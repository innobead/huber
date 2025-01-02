use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "norouter".to_string(),
        source: PackageSource::Github {
            owner: "norouter".to_string(),
            repo: "norouter".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["norouter-Linux-x86_64.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["norouter-Linux-aarch64.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["norouter-Darwin-x86_64.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["norouter-Windows-x64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
