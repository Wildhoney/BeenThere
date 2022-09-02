mod app {
    pub mod cli;
    pub mod manager;
    pub mod resources;
    pub mod utils;
    pub mod types;
    pub mod display;
}

#[tokio::main]
pub async fn main() {
    let output = app::cli::run().await;
    app::display::render(output);
}
