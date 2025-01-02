use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "onefetch".to_string(),
        source: PackageSource::Github {
            owner: "o2sh".to_string(),
            repo: "onefetch".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["onefetch-linux.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["onefetch-mac.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["onefetch-win.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
