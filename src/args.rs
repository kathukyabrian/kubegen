use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub image: String,

    #[arg(long)]
    pub port: String,

    // ClusterIP, NodePort, LoadBalancer
    #[arg(long)]
    pub service_type: String,
}