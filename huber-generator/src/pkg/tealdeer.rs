use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "tealdeer".to_string(),
        source: PackageSource::Github {
            owner: "tealdeer-rs".to_string(),
            repo: "tealdeer".to_string(),
        },
        ..Default::default()
    }
}
