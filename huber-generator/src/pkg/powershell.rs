use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "powershell".to_string(),
        source: PackageSource::Github {
            owner: "PowerShell".to_string(),
            repo: "PowerShell".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["powershell-{version}-linux-x64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["powershell-{version}-osx-x64.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
