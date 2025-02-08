use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "deno".to_string(),
        source: PackageSource::Github {
            owner: "denoland".to_string(),
            repo: "deno".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}
