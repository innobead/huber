use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "go-http-tunnel".to_string(),
        source: PackageSource::Github {
            owner: "mmatczuk".to_string(),
            repo: "go-http-tunnel".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["tunnel_linux_amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["tunnel_darwin_amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["tunnel_windows_amd64.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
