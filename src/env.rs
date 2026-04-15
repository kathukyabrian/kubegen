use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct Env {
    pub name: String,
    pub value: String,
}