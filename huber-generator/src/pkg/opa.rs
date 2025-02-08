use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "opa".to_string(),
        source: PackageSource::Github {
            owner: "open-policy-agent".to_string(),
            repo: "opa".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["opa_linux_amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["opa_darwin_amd64".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["opa_windows_amd64.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
