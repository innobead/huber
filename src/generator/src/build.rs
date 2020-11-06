use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use huber_common::model::package::{Package, PackageIndex};
use huber_common::result::Result;

use crate::pkg::*;

mod pkg;

fn main() -> Result<()> {
    let generated_dir = &Path::new(env::var("CARGO_MANIFEST_DIR")?.as_str())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("generated")
        .join("packages");

    // clean up and prepare
    fs::remove_dir_all(generated_dir.clone()).unwrap();
    fs::create_dir_all(generated_dir.clone()).unwrap();

    // generate release manifests, index file
    let index_file = Path::new(generated_dir)
        .parent()
        .unwrap()
        .join("index.yaml");
    let mut index_file = File::create(index_file)?;
    writeln!(index_file, "{}", "# This is generated. Don't modify.")?;

    let mut pkg_indexes: Vec<PackageIndex> = vec![];

    for r in releases().iter() {
        let str = format!(
            "# This is generated. Don't modify.\n{}",
            serde_yaml::to_string(&r)?
        );

        pkg_indexes.push(PackageIndex {
            name: r.name.clone(),
            owner: r.source.owner(),
            source: r.source.to_string(),
        });

        let pkg_file = Path::new(generated_dir)
            .join(r.name.clone())
            .with_extension("yaml");

        File::create(pkg_file)?.write_all(str.as_bytes())?;
    }

    writeln!(
        index_file,
        "{}",
        serde_yaml::to_string(&pkg_indexes).unwrap()
    )?;

    Ok(())
}

fn releases() -> Vec<Package> {
    vec![
        gh::release(),
        velero::release(),
        kubefire::release(),
        k3s::release(),
        rke::release(),
        rio::release(),
    ]
}
