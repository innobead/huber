use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ctlptl".to_string(),
        source: PackageSource::Github {
            owner: "tilt-dev".to_string(),
            repo: "ctlptl".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["ctlptl.{version}.linux.x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["ctlptl.{version}.mac.x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["ctlptl.{version}.windows.x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
