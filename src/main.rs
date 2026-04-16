pub mod args;
pub mod env;

use clap::Parser;
use tera::{Context, Tera};

fn main() {
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

    // get user input
    // name
    // image
    // port
    // service type - ClusterIP, NodePort, LoadBalancer

    let args = args::Args::parse();

    let mut tera = load_templates();

    let name = args.name;
    let image = args.image;
    let port = args.port;
    let service_type = args.service_type;

    validate_inputs(&name, &image, &port, &service_type);

    if service_type != "NodePort" && service_type != "ClusterIP" && service_type != "LoadBalancer" {
        panic!(
            "Unsupported service type {}. Supported service types are [NodePort, ClusterIP, LoadBalancer]",
            service_type
        );
    }

    let deployment = generate_deployment(&mut tera, &name, &image, &port);
    let service = generate_service(&mut tera, &name, &service_type, &port);

    println!("{}", deployment);
    println!("---");
    println!("{}", service);
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

fn validate_inputs(_name: &str, _image: &str, _port: &str, service_type: &str) {
    if service_type != "NodePort" && service_type != "ClusterIP" && service_type != "LoadBalancer" {
        panic!(
            "Unsupported service type '{}'. Supported service types are [NodePort, ClusterIP, LoadBalancer]",
            service_type
        );
    }
}

fn generate_deployment(tera: &Tera, name: &str, image: &str, port: &str) -> String {
    let mut context = Context::new();
    context.insert("name", name);
    context.insert("image", image);
    context.insert("port", port);
    let rendered = tera.render("deployment", &context).unwrap();
    rendered
}

fn generate_service(tera: &Tera, name: &str, service_type: &str, port: &str) -> String {
    let mut context = Context::new();
    context.insert("name", name);
    context.insert("service_type", service_type);
    context.insert("port", port);

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
