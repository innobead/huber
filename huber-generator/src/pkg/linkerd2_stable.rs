use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};
use maplit::hashmap;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "linkerd2-stable".to_string(),
        source: PackageSource::Github {
            owner: "linkerd".to_string(),
            repo: "linkerd2".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["linkerd2-cli-stable-{version}-linux-amd64".to_string()],
                executable_mappings: Some(hashmap! {
                    "linkerd2-cli-stable-{version}-linux-amd64".to_string() => "linkerd2-stable".to_string()
                }),
                tag_version_regex_template: Some(r"^stable-(\d+.\d+.\d+)$".to_string()),
                scan_dirs: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["linkerd2-cli-stable-{version}-linux-arm64".to_string()],
                executable_mappings: Some(hashmap! {
                    "linkerd2-cli-stable-{version}-linux-arm64".to_string() => "linkerd2-stable".to_string()
                }),
                tag_version_regex_template: Some(r"^stable-(\d+.\d+.\d+)$".to_string()),
                scan_dirs: None,
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["linkerd2-cli-stable-{version}-darwin".to_string()],
                executable_mappings: Some(hashmap! {
                    "linkerd2-cli-stable-{version}-darwin".to_string() => "linkerd2-stable".to_string()
                }),
                tag_version_regex_template: Some(r"^stable-(\d+.\d+.\d+)$".to_string()),
                scan_dirs: None,
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["linkerd2-cli-stable-{version}-windows.exe".to_string()],
                executable_mappings: Some(hashmap! {
                    "linkerd2-cli-stable-{version}-windows.exe".to_string() => "linkerd2-stable.exe".to_string()
                }),
                tag_version_regex_template: Some(r"^stable-(\d+.\d+.\d+)$".to_string()),
                scan_dirs: None,
            }),
        ],
        ..Default::default()
    }
}
