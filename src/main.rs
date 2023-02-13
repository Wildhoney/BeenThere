// mod cli;
// mod config;
// mod db;
// mod renderer;
// mod types;

mod app {
    pub mod cli;
    pub mod config;
    pub mod fs;
    pub mod mocks;
    pub mod renderer;
    pub mod types;
    pub mod utils;
}

#[macro_use]
extern crate lazy_static;
extern crate inflector;

#[tokio::main]
pub async fn main() {
    let output = app::cli::run().await;
    app::renderer::print(output);
}
