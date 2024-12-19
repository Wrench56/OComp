mod config;

use axum::{
    routing::{get, post},
    Router,
};
use config::load_config;
use const_format::formatcp;
use std::sync::Arc;

const SERVER_HOST: &str = "0.0.0.0";
const SERVER_PORT: u16 = 3000;
static ADDRESS: &str = formatcp!("{SERVER_HOST}:{SERVER_PORT}");

#[tokio::main]
async fn main() {
    let config = Arc::new(load_config());
    let mut app = Router::new();

    for (key, _section) in &config.sections {
        let route_path = format!("/{}", key);
        let config_clone = config.clone();

        app = app
            .route(
                &route_path,
                post({
                    let build_type = key.clone();
                    move || async move { format!("Building with {}", build_type) }
                }),
            )
            .route(
                &route_path,
                get("Please use HTTP POST to build source files"),
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
