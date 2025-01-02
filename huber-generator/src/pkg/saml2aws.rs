use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "saml2aws".to_string(),
        source: PackageSource::Github {
            owner: "Versent".to_string(),
            repo: "saml2aws".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["saml2aws_{version}_linux_amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["saml2aws_{version}_linux_arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["saml2aws_{version}_darwin_amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec!["saml2aws_{version}_darwin_arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["saml2aws_{version}_windows_amd64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
