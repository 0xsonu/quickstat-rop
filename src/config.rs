use std::fs;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub paths: Paths,
    pub lte: Template,
    pub nr: Template,
    pub nr_ebsn: Template,
    
}

#[derive(Deserialize, Debug)]
pub struct Paths {
    pub input_dir: String,
    pub output_dir: String,
}

#[derive(Deserialize, Debug)]
pub struct Template {
    pub template_dir: String,
}


pub fn load_config(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}
