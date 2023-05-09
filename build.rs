use std::{env, fs};
use std::path::Path;
use serde::Deserialize;
use cargo_metadata::MetadataCommand;
use git2::{Repository, BranchType};

const PLATFORM_REPO_URL: &str = "https://github.com/dashpay/platform.git";

const CORE_PROTO_PATH: &str = "packages/dapi-grpc/protos/core/v0";
const PLATFORM_PROTO_PATH: &str = "packages/dapi-grpc/protos/platform/v0";

#[derive(Debug, Deserialize)]
struct PackageMetadata {
    platform_branch: String,
}

#[derive(Debug, Deserialize)]
struct EnvWrapper {
    env: PackageMetadata,
}

impl From<EnvWrapper> for PackageMetadata {
    fn from(wrapper: EnvWrapper) -> Self {
        wrapper.env
    }
}

fn main() {
    // Preparation
    let path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_path = Path::new(&path).join("src");

    let meta = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .current_dir(&path)
        .exec()
        .unwrap();

    let metadata = meta.root_package().unwrap().metadata.clone();
    let metadata_wrapper: EnvWrapper = serde_json::from_value(metadata).unwrap();
    let metadata: PackageMetadata = metadata_wrapper.env;
    let platform_branch = metadata.platform_branch;

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let repo_path = out_path.join("repo");

    println!("outdir: {}", out_dir);

    // Initialize repo
    let repo: Repository;
    if !repo_path.exists() {
        repo = Repository::clone(PLATFORM_REPO_URL, &repo_path).unwrap();
    }else{
        repo = Repository::open(&repo_path).unwrap();
    }

    // Checkout platform_branch
    let branch_name = platform_branch.as_str();
    let branch = repo
        .find_branch(branch_name, BranchType::Local)
        .or_else(|_| repo.find_branch(branch_name, BranchType::Remote))
        .unwrap();

    let reference = branch.into_reference();
    let object = reference.peel_to_commit().unwrap().into_object();
    repo.checkout_tree(&object, None).unwrap();
    repo.set_head(reference.name().unwrap()).unwrap();

    // Create folders
    let platform_out_dir = src_path.join("stubs/platform");

    fs::create_dir_all(&platform_out_dir).unwrap();

    let core_out_dir = src_path.join("stubs/core");
    fs::create_dir_all(&core_out_dir).unwrap();

    // Generate stubs
    let core_proto_dir = repo_path.join(CORE_PROTO_PATH);
    tonic_build::configure()
        .build_server(false)
        .out_dir(&core_out_dir)
        .compile(
            &[core_proto_dir.join("core.proto")],
            &[core_proto_dir],
        )
        .unwrap();

    let platform_proto_dir = repo_path.join(PLATFORM_PROTO_PATH);
    tonic_build::configure()
        .build_server(false)
        .out_dir(&platform_out_dir)
        .compile(
            &[platform_proto_dir.join("platform.proto")],
            &[platform_proto_dir],
        )
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}