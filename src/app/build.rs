use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde_yaml::Error;

use huber_common::model::release::{Release, ReleaseManagement, ReleaseTargetType, ReleaseType};
use huber_common::result::Result;
use huber_release::pkg::gh;

fn main() -> Result<()> {
    let generated_dir = &Path::new(env::var("CARGO_MANIFEST_DIR")?.as_str())
        .parent().unwrap().parent().unwrap()
        .join("generated")
        .join("managed_packages");

    fs::remove_dir_all(generated_dir.clone());
    fs::create_dir_all(generated_dir.clone());

    for r in releases().iter() {
        let str = format!("# This is generated. Don't modify.\n{}", serde_yaml::to_string(&r)?);
        let f = Path::new(generated_dir)
            .join(r.name.clone())
            .with_extension("yaml");
        File::create(f)?.write_all(str.as_bytes())?;
    }

    Ok(())
}

fn releases() -> Vec<Release> {
    vec![
        gh::release(),
    ]
}
