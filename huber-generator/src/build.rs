use std::env;
use std::path::Path;
use std::process::Command;

use huber_common::model::package::{Package, PackageIndex, PackageSource};
use tokio::fs::{create_dir_all, remove_file, File};
use tokio::io::AsyncWriteExt;

use crate::pkg::*;

mod pkg;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pkg_dir = env!("CARGO_MANIFEST_DIR");

    let generated_dir = &Path::new(&pkg_dir)
        .parent()
        .unwrap()
        .join("generated")
        .join("packages");

    let force_generated: bool = env::var("FORCE_GENERATE")
        .unwrap_or_else(|_| {
            if generated_dir.exists() {
                "false"
            } else {
                "true"
            }
            .to_string()
        })
        .parse()?;

    if force_generated {
        println!("Force generate packages in {:?}", generated_dir);
    } else {
        println!("Only generate the impacted packages in {:?}", generated_dir);
    }

    // clean up and prepare
    // remove_dir_all(generated_dir.clone()).unwrap();
    create_dir_all(generated_dir.clone()).await?;

    // generate release manifests, index file
    let index_file = Path::new(generated_dir)
        .parent()
        .unwrap()
        .join("index.yaml");

    let _ = remove_file(&index_file).await;
    let mut index_file = File::create(index_file).await?;
    index_file
        .write_all("# This is generated. Don't modify.\n".as_bytes())
        .await?;

    let mut pkg_indexes: Vec<PackageIndex> = vec![];
    for mut pkg in releases().into_iter() {
        pkg_indexes.push(PackageIndex {
            name: pkg.name.clone(),
            owner: pkg.source.owner(),
            source: pkg.source.to_string(),
        });

        if !force_generated {
            let pkg_rs_file = Path::new(&pkg_dir)
                .join("src")
                .join("pkg")
                .join(format!("{}.rs", pkg.name));

            // This is the best effort to check for any local changes,
            // except that the change has been pushed to the remote origin.
            let pkg_rs_file_changed = [
                "diff --exit-code --quiet",
                "diff --exit-code --quiet --cached",
                "diff --exit-code --quiet origin/main",
            ]
            .iter()
            .any(|args| {
                let mut args: Vec<&str> = args.split(' ').collect();
                args.append(&mut vec!["--", pkg_rs_file.to_str().unwrap()]);

                Command::new("git")
                    .args(&args)
                    .output()
                    .map(|output| output.status.code().unwrap() != 0)
                    .unwrap()
            });

            if !pkg_rs_file_changed {
                continue;
            }

            println!("{:?} changed", pkg_rs_file)
        }

        update_description(&mut pkg).await?;

        let str = format!(
            "# This is generated. Don't modify.\n{}",
            serde_yaml::to_string(&pkg)?
        );

        let pkg_file = Path::new(generated_dir)
            .join(pkg.name.clone())
            .with_extension("yaml");

        File::create(pkg_file)
            .await?
            .write_all(str.as_bytes())
            .await?;
    }

    pkg_indexes.sort_by(|x, y| x.name.partial_cmp(&y.name).unwrap());
    index_file
        .write_all(serde_yaml::to_string(&pkg_indexes).unwrap().as_bytes())
        .await?;

    Ok(())
}

fn releases() -> Vec<Package> {
    vec![
        // tools
        jiq::release(),
        jless::release(),
        onefetch::release(),
        renote::release(),
        gh::release(),
        dasel::release(),
        bat::release(),
        terraform::release(),
        packer::release(),
        hub::release(),
        syncthing::release(),
        powershell::release(),
        mkcert::release(),
        huber::release(),
        ali::release(),
        gping::release(),
        gitui::release(),
        ripgrep::release(),
        starship::release(),
        tokei::release(),
        exa::release(),
        fd::release(),
        procs::release(),
        k6::release(),
        fortio::release(),
        jwt_cli::release(),
        direnv::release(),
        tracee::release(),
        vegeta::release(),
        yq::release(),
        stern::release(),
        drone_cli::release(),
        img::release(),
        dive::release(),
        wasme::release(),
        hyperfine::release(),
        hetty::release(),
        czkawka::release(),
        cloak::release(),
        jq::release(),
        termshark::release(),
        volta::release(),
        just::release(),
        croc::release(),
        terrascan::release(),
        tecli::release(),
        nerdctl::release(),
        zoxide::release(),
        dust::release(),
        pueue::release(),
        coreutils::release(),
        hugo::release(),
        typos::release(),
        zellij::release(),
        xh::release(),
        loc::release(),
        choose::release(),
        delta::release(),
        dog::release(),
        dua_cli::release(),
        skim::release(),
        hexyl::release(),
        lsd::release(),
        fnm::release(),
        frum::release(),
        nat::release(),
        sad::release(),
        sd::release(),
        navi::release(),
        tealdeer::release(),
        bottom::release(),
        grex::release(),
        codeql::release(),
        viddy::release(),
        cosign::release(),
        saml2aws::release(),
        grpcurl::release(),
        buf::release(),
        shisho::release(),
        // infra
        pulumi::release(),
        chisel::release(),
        go_http_tunnel::release(),
        shadowsocks::release(),
        norouter::release(),
        wstunnel::release(),
        doctl::release(),
        zola::release(),
        nushell::release(),
        dolt::release(),
        // app development
        okteto::release(),
        skaffold::release(),
        kpt::release(),
        oras::release(),
        waypoint::release(),
        tilt::release(),
        ko::release(),
        protoc::release(),
        copilot_cli::release(),
        rustwasmc::release(),
        // cloud native, kubernetes
        rancher::release(),
        rancher_cli::release(),
        velero::release(),
        helm::release(),
        helmfile::release(),
        kubefire::release(),
        kubectl::release(),
        kustomize::release(),
        k3s::release(),
        k3sup::release(),
        k3d::release(),
        rke::release(),
        rke2::release(),
        rio::release(),
        istio::release(),
        fleet::release(),
        kube_bench::release(),
        trivy::release(),
        octant::release(),
        pack::release(),
        opa::release(),
        conftest::release(),
        submariner::release(),
        kind::release(),
        krew::release(),
        minikube::release(),
        sonobuoy::release(),
        consul::release(),
        ctlptl::release(),
        arkade::release(),
        faas_cli::release(),
        fission::release(),
        k9s::release(),
        lens::release(),
        k0s::release(),
        kudo::release(),
        kuttl::release(),
        flux2::release(),
        argocd::release(),
        ketch::release(),
        kompose::release(),
        eksctl::release(),
        linkerd2_edge::release(),
        linkerd2_stable::release(),
        camel_k::release(),
        keptn::release(),
        kubevirt::release(),
        kubestr::release(),
        kube_linter::release(),
        natscli::release(),
        kwctl::release(),
        // runtime
        containerd::release(),
        firecracker::release(),
        nomad::release(),
        podman::release(),
        wasmtime::release(),
        wasmer::release(),
        ssvm::release(),
        wasm_to_oci::release(),
        wabt::release(),
        // programming
        deno::release(),
        typescript::release(),
        node::release(),
        kotlin::release(),
        gradle::release(),
        // blockchain
        solidity::release(),
        axelard::release(),
        // ai
        ollama::release(),
    ]
}

async fn update_description(pkg: &mut Package) -> anyhow::Result<()> {
    println!("Updating the description of package: {}", pkg);

    let octocrab = octocrab::OctocrabBuilder::default()
        .personal_token(env::var("GITHUB_TOKEN")?)
        .build()?;

    let PackageSource::Github { owner, repo } = &pkg.source;
    let repo = octocrab.repos(owner, repo).get().await?;
    pkg.description = repo.description;

    Ok(())
}
