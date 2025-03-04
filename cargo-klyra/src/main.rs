use anyhow::Result;
use cargo_klyra::{Args, Klyra};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    Klyra::new().run(Args::from_args()).await
}
