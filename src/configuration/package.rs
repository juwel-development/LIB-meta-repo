use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Package {
    pub dir: String,
    pub git: String,
}