use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Address to bind the control plane to
    #[clap(
        long,
        default_value = "127.0.0.1:8001"
    )]
    pub control: SocketAddr,
    /// Address to bind the user plane to
    #[clap(
        long,
        default_value = "127.0.0.1:8000"
    )]
    pub user: SocketAddr,
    /// Default image to deploy user runtimes into
    #[clap(
        long,
        default_value = "public.ecr.aws/d7w6e9t1/backend:latest"
    )]
    pub image: String,
    /// Prefix to add to the name of all docker resources managed by
    /// this service
    #[clap(
        long,
        default_value = "klyra_prod_"
    )]
    pub prefix: String,
    /// The address at which an active runtime container will find
    /// the provisioner service
    #[clap(
        long,
        default_value = "provisioner"
    )]
    pub provisioner_host: String,
    /// The Docker Network ID in which to deploy user runtimes
    #[clap(long)]
    pub network_id: String
}
