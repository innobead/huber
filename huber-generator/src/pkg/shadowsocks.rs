use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "shadowsocks".to_string(),
        source: PackageSource::Github {
            owner: "shadowsocks".to_string(),
            repo: "shadowsocks-rust".to_string(),
        },
        ..Default::default()
    }
}
