mod builder;
mod config;

use axum::{
    extract::Multipart,
    routing::{get, post},
    Router,
};
use const_format::formatcp;
use std::sync::Arc;

const SERVER_HOST: &str = "0.0.0.0";
const SERVER_PORT: u16 = 3000;
static ADDRESS: &str = formatcp!("{SERVER_HOST}:{SERVER_PORT}");

#[tokio::main]
async fn main() {
    let config = Arc::new(config::load_config());
    builder::init().await;

    let mut app = Router::new();

    for (key, section) in &config.sections {
        let route_path = format!("/{}", key);
        let _config_clone = config.clone();
        let build_command = section.build.clone();

        app = app
            .route(
                &route_path,
                post({
                    let build_command = build_command.clone();
                    move |multipart: Multipart| {
                        let build_command = build_command.clone();
                        async move {
                            match builder::build(&build_command, multipart).await {
                                Ok(output) => format!("Build success: \n{}", output),
                                Err(err) => format!("Build failed: \n{}", err),
                            }
                        }
                    }
                }),
            )
            .route(
                &route_path,
                get("Please use HTTP POST with a source file to build"),
            );
    }

    println!("Starting server on {ADDRESS}");
    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Shutdown signal handler failed");
    println!("Bye-bye!");
}
