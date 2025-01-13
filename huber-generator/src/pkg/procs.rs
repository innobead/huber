use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "procs".to_string(),
        source: PackageSource::Github {
            owner: "dalance".to_string(),
            repo: "procs".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["procs-v{version}-x86_64-lnx.zip".to_string()],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["procs-v{version}-x86_64-mac.zip".to_string()],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["procs-v{version}-x86_64-win.zip".to_string()],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
        ],
        version: None,
        description: None,
        release_kind: None,
    }
}
