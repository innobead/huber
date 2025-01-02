use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "hetty".to_string(),
        source: PackageSource::Github {
            owner: "dstotijn".to_string(),
            repo: "hetty".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["hetty_{version}_Linux_x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["hetty_{version}_macOS_x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["hetty_{version}_Windows_x86_64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
