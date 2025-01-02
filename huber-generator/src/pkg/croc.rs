use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "croc".to_string(),
        source: PackageSource::Github {
            owner: "schollz".to_string(),
            repo: "croc".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["croc_{version}_Linux-64bit.tar.gz".to_string()],
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["croc_{version}_Linux-ARM64.tar.gz".to_string()],
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["croc_{version}_macOS-64bit.tar.gz".to_string()],
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["croc_{version}_Windows-64bit.zip".to_string()],
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
        ],
        ..Default::default()
    }
}
