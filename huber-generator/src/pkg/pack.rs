use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "pack".to_string(),
        source: PackageSource::Github {
            owner: "buildpacks".to_string(),
            repo: "pack".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["pack-v{version}-linux.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxAmd64(Default::default()),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["pack-v{version}-macos.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(Default::default()),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["pack-v{version}-windows.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
