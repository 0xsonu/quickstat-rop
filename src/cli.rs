use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "stats-replay")]
pub struct Cli {
    #[arg(short, long, default_value = "app_config.toml")]
    pub config_path: String,

    #[arg(short, long)]
    pub rop: Option<String>,

    #[arg(short, long)]
    pub input: Option<String>,

    #[arg(short, long)]
    pub output: Option<String>,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
