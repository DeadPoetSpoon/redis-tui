use clap::Parser;

/// Toy TUI for Redis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    /// redis url, eg. redis://default:******@127.0.0.1:6379.
    #[arg(short,long)]
    pub url: Vec<String>,
}