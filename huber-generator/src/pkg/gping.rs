use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "gping".to_string(),
        source: PackageSource::Github {
            owner: "orf".to_string(),
            repo: "gping".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::LinuxArm64(Default::default()),
            PackageTargetType::LinuxArm(Default::default()),
            PackageTargetType::MacOSAmd64(Default::default()),
            PackageTargetType::MacOSArm64(Default::default()),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["gping-Windows-msvc-x86_64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
