use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "rclone".to_string(),
        source: PackageSource::Github {
            owner: "rclone".to_string(),
            repo: "rclone".to_string(),
        },
        ..Default::default()
    }
}
