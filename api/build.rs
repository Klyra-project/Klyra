use std::path::Path;

use klyra_common::version::get_klyra_service_from_user_crate;

fn main() {
    let version = get_klyra_service_from_user_crate(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
    )
    .unwrap();
    println!(
        "cargo:rustc-env=klyra_SERVICE_VERSION_REQ=^{}.{}",
        version.major, version.minor,
    );
}
