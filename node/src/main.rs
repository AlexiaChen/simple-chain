use cli::Cli;

#[tokio::main]
pub async fn main() {
    let cli = Cli::new();
    cli.run();
}
