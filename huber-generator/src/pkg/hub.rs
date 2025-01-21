use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};
use maplit::hashmap;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "hub".to_string(),
        source: PackageSource::Github {
            owner: "github".to_string(),
            repo: "hub".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["hub-linux-amd64-{version}.tgz".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "install".to_string() => "hub-install".to_string()
                }),
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["hub-linux-arm64-{version}.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["hub-darwin-amd64-{version}.tgz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["hub-windows-amd64-{version}.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}
