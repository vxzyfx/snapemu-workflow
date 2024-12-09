

use devices_manager::run;
use std::env;
use clap::{Command, FromArgMatches as _, Parser, Subcommand as _};

#[derive(Parser, Debug)]
enum Subcommands {
    Run {
        #[arg(short, long, default_value="/etc/snapemu/config.yaml", env="SNAPEMU_CONFIG")]
        config: String,
        #[arg(short, long, default_value="SNAPEMU_DEVICE_", env="SNAPEMU_DEVICE_ENV_PREFIX")]
        env_prefix: String,
    },
}

fn cmd() -> Command {
    let cli = Command::new("snapemu-manager")
        .version(env!("SNAPEMU_DEVICE_VERSION"));
    Subcommands::augment_subcommands(cli)

}
#[tokio::main]
async fn main() {
    let cli = cmd();
    let matches = cli.get_matches();
    match Subcommands::from_arg_matches(&matches) {
        Ok(subcommand) => {
            match subcommand {
                Subcommands::Run { config, env_prefix } => {
                    run(config, env_prefix).await;
                }
            }
        }
        Err(_) => {
            let mut cli = cmd();
            cli.print_help().unwrap()
        }
    }
}
