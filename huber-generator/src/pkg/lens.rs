use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "lens".to_string(),
        source: PackageSource::Github {
            owner: "lensapp".to_string(),
            repo: "lens".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["Lens-{version}.AppImage".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["Lens-{version}.dmg".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["Lens-Setup-{version}.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
