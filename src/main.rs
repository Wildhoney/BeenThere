mod app {
    pub mod cli;
    pub mod manager;
    pub mod resources;
    pub mod utils;
    pub mod types;
}

#[tokio::main]
pub async fn main() {
    let x = app::cli::run().await;
    println!("{:?}", x);
}
