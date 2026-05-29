use std::process::exit;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "http://localhost:3004/health")]
    url: String,

    #[arg(short, long)]
    load: String,

    #[arg(short, long)]
    duration: String,
}

#[tokio::main]
async fn main() {
    let _args = Args::parse();

    match verify_url_is_reachable(&_args.url).await {
        Ok(_) => println!("✅ {} is Reachable", _args.url),
        Err(_) => {
            println!("❌ {} is Unreachable", _args.url);
            println!("Stopping Load tester...");
            exit(1)
        }
    }
}

async fn verify_url_is_reachable(url: &str) -> reqwest::Result<reqwest::Response> {
    println!("Verifying URL is reachable...");
    reqwest::get(url).await
}
