use crate::config::{generate_schema, parse_config};
use axum::{ middleware, routing::post, Router};
use clap::{Parser, Subcommand};
use config::Config;
use handler::{print_request_response, ReportHandler, handler};
use reporter::Reporter;
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;
pub mod config;
pub mod handler;
pub mod reporter; 

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliFlags {
    /// The configuration file to use; this is required and can be relative.
    #[clap(long = "config", short, default_value = "config.yaml")]
    config_path: String,

    #[clap(long, short, default_value = "false")]
    debug: bool,

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
    let args = CliFlags::parse();
    
    // Parse the configuration file and load it if needed
    let mut user_config = parse_config(args.config_path.as_str());

    if user_config.interval == 0 {
        warn!("Interval is set to 0 or less; defaulting to 5 seconds");
        user_config.interval = 5;
    }
    
    let level = if args.debug {
        Level::DEBUG
    } else {
        Level::INFO
    };

    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    let reporter = Reporter::new(user_config.clone());
    // This shouldn't fail, hence the .expect()
    tracing::subscriber::set_global_default(subscriber).expect("setting default logger failed");

    // Parse the command line arguments
    let opt = CliFlags::parse();

    match opt.command {
        Some(Commands::ConfigSchema) => {
            generate_schema();
        }
        None => {
            start_server(reporter, user_config, args.debug).await;
        },
    }
}

async fn start_server(reporter: Reporter, config: Config, debug: bool) {
    info!("Starting server...");
    let report_handler = ReportHandler { reporter };
    let mut app = Router::new()
        // In Axum, we can't have a wildcard that also applies to the root, so we have to define this twice. 
        // Source: https://docs.rs/axum/latest/axum/routing/struct.Router.html#wildcards 
        .route("/", post(handler).with_state(report_handler.clone()))
        .route("/*{x}", post(handler).with_state(report_handler.clone()));
        
    if debug {
        app = app.layer(middleware::from_fn(print_request_response));
    }

    let listener = tokio::net::TcpListener::bind(config.listen)
        .await
        .unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
