use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "natscli".to_string(),
        source: PackageSource::Github {
            owner: "nats-io".to_string(),
            repo: "natscli".to_string(),
        },
        ..Default::default()
    }
}
