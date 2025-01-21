use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "procs".to_string(),
        source: PackageSource::Github {
            owner: "dalance".to_string(),
            repo: "procs".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["procs-v{version}-x86_64-lnx.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["procs-v{version}-x86_64-mac.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["procs-v{version}-x86_64-win.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
