use crate::config::generate_schema;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
pub mod config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliFlags {
    /// The configuration file to use; this is required and can be relative.
    #[clap(long = "config", short, default_value = "config.yaml")]
    config_path: String,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ConfigSchema,
}

#[tokio::main]
/// Entrypoint into the application.
async fn main() {
    // Parse the configuration file and load it if needed
    // let user_config = parse_config(args.config_path.as_str());

    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();

    // This shouldn't fail, hence the .expect()
    tracing::subscriber::set_global_default(subscriber).expect("setting default logger failed");

    // Parse the command line arguments
    let opt = CliFlags::parse();

    match opt.command {
        Some(Commands::ConfigSchema) => {
            generate_schema();
        }
        None => {
            info!("Hello, world!");
        },
    }
}
