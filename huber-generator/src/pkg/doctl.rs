use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "doctl".to_string(),
        source: PackageSource::Github {
            owner: "digitalocean".to_string(),
            repo: "doctl".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}
