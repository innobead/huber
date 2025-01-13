use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "k9s".to_string(),
        source: PackageSource::Github {
            owner: "derailed".to_string(),
            repo: "k9s".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "k9s_Linux_amd64.tar.gz".to_string(),
                    "k9s_v{version}_Linux_x86_64.tar.gz".to_string(),
                    "{version}/k9s_Linux_x86_64.tar.gz".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "k9s_Linux_arm64.tar.gz".to_string(),
                    "k9s_v{version}_Linux_arm64.tar.gz".to_string(),
                    "{version}/k9s_Linux_arm64.tar.gz".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "k9s_Darwin_amd64.tar.gz".to_string(),
                    "k9s_v{version}_Darwin_x86_64.tar.gz".to_string(),
                    "{version}/k9s_Darwin_x86_64.tar.gz".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec!["k9s_Darwin_arm64.tar.gz".to_string()],
                executable_templates: None,
                executable_mappings: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "k9s_Windows_amd64.tar.gz".to_string(),
                    "k9s_v{version}_Windows_x86_64.tar.gz".to_string(),
                    "{version}/k9s_Windows_x86_64.tar.gz".to_string(),
                ],
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
