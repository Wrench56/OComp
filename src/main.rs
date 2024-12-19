mod config;

const CURRENT_CONFIG_VERSION: &str = "1.0.0";

fn main() {
    let config = config::load_config();
    println!("The config is: {:?}", config);
}
