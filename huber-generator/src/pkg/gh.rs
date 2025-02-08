use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "gh".to_string(),
        source: PackageSource::Github {
            owner: "cli".to_string(),
            repo: "cli".to_string(),
        },
        ..Default::default()
    }
}
