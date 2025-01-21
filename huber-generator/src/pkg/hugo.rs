use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "hugo".to_string(),
        source: PackageSource::Github {
            owner: "gohugoio".to_string(),
            repo: "hugo".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "hugo_{version}_Linux-64bit.tar.gz".to_string(),
                    "hugo_extended_{version}_Linux-64bit.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "hugo_{version}_macOS-64bit.tar.gz".to_string(),
                    "hugo_extended_{version}_macOS-64bit.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "hugo_{version}_Windows-64bit.zip".to_string(),
                    "hugo_extended_{version}_Windows-64bit.zip".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
