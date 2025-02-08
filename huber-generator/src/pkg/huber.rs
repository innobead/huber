use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "huber".to_string(),
        source: PackageSource::Github {
            owner: "innobead".to_string(),
            repo: "huber".to_string(),
        },
        ..Default::default()
    }
}
