use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "tilt".to_string(),
        source: PackageSource::Github {
            owner: "tilt-dev".to_string(),
            repo: "tilt".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["tilt.{version}.linux.x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["tilt.{version}.mac.x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["tilt.{version}.windows.x86_64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
