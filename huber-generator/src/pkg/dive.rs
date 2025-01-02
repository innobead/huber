use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "dive".to_string(),
        source: PackageSource::Github {
            owner: "wagoodman".to_string(),
            repo: "dive".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["dive_{version}_linux_amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["dive_{version}_darwin_amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["dive_{version}_windows_amd64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
