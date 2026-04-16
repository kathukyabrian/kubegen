use serde::Serialize;

#[derive(clap::ValueEnum, Clone, Serialize)]
pub enum ServiceType {
    #[value(name = "ClusterIP")]
    ClusterIP,
    #[value(name = "NodePort")]
    NodePort,
    #[value(name = "LoadBalancer")]
    LoadBalancer,
}