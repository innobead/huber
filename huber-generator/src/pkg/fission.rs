use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};
use maplit::hashmap;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "fission".to_string(),
        source: PackageSource::Github {
            owner: "fission".to_string(),
            repo: "fission".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "{version}/fission-cli-linux".to_string(),
                    "fission-all-{version}.tgz".to_string(),
                ],
                executable_mappings: Some(hashmap! {
                    "fission-cli-linux".to_string() => "fission".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "{version}/fission-cli-osx".to_string(),
                    "fission-all-{version}.tgz".to_string(),
                ],
                executable_mappings: Some(hashmap! {
                    "fission-cli-osx".to_string() => "fission".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "{version}/fission-cli-windows.exe".to_string(),
                    "fission-all-{version}.tgz".to_string(),
                ],
                executable_mappings: Some(hashmap! {
                    "fission-cli-windows.exe".to_string() => "fission.exe".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
        ],
        ..Default::default()
    }
}
