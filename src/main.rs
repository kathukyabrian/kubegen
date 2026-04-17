pub mod args;
pub mod servicetype;

use crate::args::Args;
use clap::Parser;
use tera::{Context, Tera};

fn main() {
    print_banner();

    let args = args::Args::parse();

    let tera = load_templates();

    let deployment = generate_deployment(&tera, &args);
    let service = generate_service(&tera, &args);
    let ingress = generate_ingress(&tera, &args);
    let configmap = generate_config_map(&tera, &args);

    println!("{}", deployment);
    println!("---");
    println!("{}", service);
    println!("---");
    println!("{}", ingress);
    println!("---");
    println!("{}", configmap);
}

fn print_banner() {
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
    tera.add_raw_template(
        "deployment",
        include_str!("../templates/deployment.yaml.tera"),
    )
    .unwrap();
    tera.add_raw_template("service", include_str!("../templates/service.yaml.tera"))
        .unwrap();
    tera.add_raw_template("ingress", include_str!("../templates/ingress.yaml.tera"))
        .unwrap();
    tera.add_raw_template(
        "configmap",
        include_str!("../templates/configmap.yaml.tera"),
    )
    .unwrap();

    tera
}

fn generate_deployment(tera: &Tera, args: &Args) -> String {
    let mut context = Context::new();
    context.insert("name", &args.name);
    context.insert("image", &args.image);
    context.insert("port", &args.port);

    render("deployment", &tera, &context)
}

fn generate_service(tera: &Tera, args: &Args) -> String {
    let mut context = Context::new();
    context.insert("name", &args.name);
    context.insert("service_type", &args.service_type);
    context.insert("port", &args.port);

    render("service", &tera, &context)
}

fn generate_ingress(tera: &Tera, args: &Args) -> String {
    let mut context = Context::new();
    context.insert("name", &args.name);
    context.insert("port", &args.port);
    context.insert("host", &args.host);
    context.insert("certificate_issuer", &args.certificate_issuer);

    render("ingress", &tera, &context)
}

fn generate_config_map(tera: &Tera, args: &Args) -> String {
    let mut context = Context::new();
    context.insert("name", &args.name);

    render("configmap", &tera, &context)
}

fn render(template_name: &str, tera: &Tera, context: &Context) -> String {
    let rendered = tera.render(template_name, &context).unwrap();
    rendered
}
