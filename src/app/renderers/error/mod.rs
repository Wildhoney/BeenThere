use colored::*;

pub fn render(message: &str) {
    let title = "Oh no! An error has occurred...\n".bright_red().bold();
    println!("{title}");
    println!("{message}");
}
