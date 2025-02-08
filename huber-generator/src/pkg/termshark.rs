use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "termshark".to_string(),
        source: PackageSource::Github {
            owner: "gcla".to_string(),
            repo: "termshark".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}
