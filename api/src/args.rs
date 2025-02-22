use std::net::IpAddr;
use std::path::PathBuf;

use klyra_common::Port;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "klyra")]
pub struct Args {
    #[structopt(long, about = "Override the default root path for klyra")]
    pub(crate) path: Option<PathBuf>,
    #[structopt(
        long,
        about = "Override the default port for the proxy",
        default_value = "8000"
    )]
    pub(crate) proxy_port: Port,
    #[structopt(
        long,
        about = "Override the default port for the api",
        default_value = "8001"
    )]
    pub(crate) api_port: Port,
    #[structopt(
        long,
        about = "Override the default bind address",
        default_value = "127.0.0.1"
    )]
    pub(crate) bind_addr: IpAddr,
}
