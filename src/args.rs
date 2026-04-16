use clap::Parser;
use crate::servicetype::ServiceType;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub image: String,

    #[arg(long)]
    pub port: u16,

    // ClusterIP, NodePort, LoadBalancer
    #[arg(long, value_enum)]
    pub service_type: ServiceType,
}