use quickstat_rop::{cli::Cli, config::load_config};

fn main() {
    let cli = Cli::parse();
    let config = load_config(&cli.config_path).expect("Failed to load configuration");

    println!("{config:?}");
}
