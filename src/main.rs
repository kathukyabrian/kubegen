pub mod args;
pub mod env;
pub mod servicetype;

use clap::Parser;
use tera::{Context, Tera};
use crate::args::Args;

fn main() {

    print_banner();
    // get user input
    // name
    // image
    // port
    // service type - ClusterIP, NodePort, LoadBalancer

    let args = args::Args::parse();

    let tera = load_templates();

    // validate_inputs(&name, &image, &port, &service_type);

    let deployment = generate_deployment(&tera, &args);
    let service = generate_service(&tera, &args);

    println!("{}", deployment);
    println!("---");
    println!("{}", service);
}

fn print_banner(){
    println!(
    r#"
    ██╗  ██╗██╗   ██╗██████╗ ███████╗ ██████╗ ███████╗███╗   ██╗
    ██║ ██╔╝██║   ██║██╔══██╗██╔════╝██╔════╝ ██╔════╝████╗  ██║
    █████╔╝ ██║   ██║██████╔╝█████╗  ██║  ███╗█████╗  ██╔██╗ ██║
    ██╔═██╗ ██║   ██║██╔══██╗██╔══╝  ██║   ██║██╔══╝  ██║╚██╗██║
    ██║  ██╗╚██████╔╝██████╔╝███████╗╚██████╔╝███████╗██║ ╚████║
    ╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚══════╝ ╚═════╝ ╚══════╝╚═╝  ╚═══╝

            Generate Kubernetes YAML like a boss
    "#
    );
}

fn load_templates() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_template("deployment",include_str!("../templates/deployment.yaml.tera"))
        .unwrap();
    tera.add_raw_template( "service", include_str!("../templates/service.yaml.tera"))
        .unwrap();
    tera.add_raw_template( "ingress", include_str!("../templates/ingress.yaml.tera"))
        .unwrap();
    tera.add_raw_template( "configmap", include_str!("../templates/configmap.yaml.tera"))
        .unwrap();

    tera
}

fn generate_deployment(tera: &Tera, args: &Args) -> String {
    let mut context = Context::new();
    context.insert("name", &args.name);
    context.insert("image", &args.image);
    context.insert("port", &args.port);
    let rendered = tera.render("deployment", &context).unwrap();
    rendered
}

fn generate_service(tera: &Tera, args: &Args) -> String {
    let mut context = Context::new();
    context.insert("name", &args.name);
    context.insert("service_type", &args.service_type);
    context.insert("port", &args.port);

    let rendered = tera.render("service", &context).unwrap();
    rendered
}
//
// fn generateIngress() {
//
// }
//
// fn generateConfigMap(){
//
// }
