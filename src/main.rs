pub mod env;

use tera::{Context, Tera};

fn main() {
    println!(r#"
        ██╗  ██╗██╗   ██╗██████╗ ███████╗ ██████╗ ███████╗███╗   ██╗
        ██║ ██╔╝██║   ██║██╔══██╗██╔════╝██╔════╝ ██╔════╝████╗  ██║
        █████╔╝ ██║   ██║██████╔╝█████╗  ██║  ███╗█████╗  ██╔██╗ ██║
        ██╔═██╗ ██║   ██║██╔══██╗██╔══╝  ██║   ██║██╔══╝  ██║╚██╗██║
        ██║  ██╗╚██████╔╝██████╔╝███████╗╚██████╔╝███████╗██║ ╚████║
        ╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚══════╝ ╚═════╝ ╚══════╝╚═╝  ╚═══╝

                Generate Kubernetes YAML like a boss
    "#);

    // get user input
    // name
    // image

    let mut tera = Tera::default();
    tera.add_template_file("deployment.yaml.tera", Some("deployment"))
        .unwrap();

    let deployment = generate_deployment(&mut tera, "mobireg", "mobi");

    println!("{}", deployment);
}

fn generate_deployment(tera: &Tera, name: &str, image: &str) -> String {
    let mut context = Context::new();
    context.insert("name", name);
    context.insert("image", image);
    let rendered = tera.render("deployment", &context).unwrap();
    rendered
}

// fn generateService() -> &str {
//
// }
//
// fn generateIngress() {
//
// }
//
// fn generateConfigMap(){
//
// }
