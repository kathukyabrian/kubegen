pub mod env;

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

    let mut tera = load_templates();

    let deployment = generate_deployment(&mut tera, "mobireg", "mobi", "3000");
    let service = generate_service(&mut tera, "mobireg", "NodePort", "3000");

    println!("{}", deployment);
    println!("---");
    println!("{}", service);
}

fn load_templates() -> Tera {
    let mut tera = Tera::default();
    tera.add_template_file("deployment.yaml.tera", Some("deployment"))
        .unwrap();
    tera.add_template_file("service.yaml.tera", Some("service"))
        .unwrap();
    tera.add_template_file("ingress.yaml.tera", Some("ingress"))
        .unwrap();
    tera.add_template_file("configmap.yaml.tera", Some("configmap"))
        .unwrap();

    tera
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
