use anyhow::Result;
use cargo_klyra::{parse_args, setup_tracing, Binary, Klyra};

#[tokio::main]
async fn main() -> Result<()> {
    let (args, provided_path_to_init) = parse_args();

    setup_tracing(args.debug);

    Klyra::new(Binary::CargoKlyra)?
        .run(args, provided_path_to_init)
        .await
}
