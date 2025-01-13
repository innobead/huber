use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "nushell".to_string(),
        source: PackageSource::Github {
            owner: "nushell".to_string(),
            repo: "nushell".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["nu_{version:_}_linux.tar.gz".to_string()],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: Some(vec!["nushell-{version}".to_string()]),
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["nu_{version:_}_macOS.zip".to_string()],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: Some(vec!["nushell-{version}".to_string()]),
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["nu_{version:_}_windows.zip".to_string()],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: Some(vec!["nushell-{version}".to_string()]),
            }),
        ],
        version: None,
        description: None,
        release_kind: None,
    }
}
