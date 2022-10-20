mod app {
    pub mod cli;
    pub mod fs;
    pub mod mocks;
    pub mod types;
    pub mod utils;

    pub mod renderer;
    pub mod renderers {
        pub mod error;
        pub mod info;
        pub mod list;
        pub mod modified;
    }
}

#[tokio::main]
pub async fn main() {
    let output = app::cli::run().await;
    app::renderer::print(output);
}
