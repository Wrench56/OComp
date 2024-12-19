mod config;

fn main() {
    let config = config::load_config();
    println!("The config is: {:?}", config);
}
