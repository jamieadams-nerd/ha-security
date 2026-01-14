mod model;
mod mock;
mod output;
mod config;
mod config_loader;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "/etc/umrs/logspace.toml")]
    config: String,

    #[arg(long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();

    let config = config_loader::load_config(&cli.config)
        .expect("configuration error");

    // TEMP: still using mock data
    let pools = mock::sample_pools();

    if cli.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&pools).unwrap()
        );
    } else {
        output::print_pools(&pools);
    }

    // config is loaded and validated — but not yet applied
    let _ = config;
}
