mod app_show;
mod app_watch;
mod config;
mod feed_fetch;
mod feed_parse;
mod feed_formatter;

use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args.get(1);

    let config = config::load_config();

    match command {
        Some(command) => match command.as_str() {
            "watch" => {
                app_watch::app_main(&args, &config).await;
                return;
            }
            "show" => {
                app_show::app_main(&args, &config).await;
                return;
            }
            _ => {}
        },
        None => {}
    }

    print_help();
}

fn print_help() {
    println!("{}", env!("CARGO_PKG_NAME"));
    println!("");
    println!("Commands:");
    println!("  show      Show all feed.");
    println!("  watch     Start watching feed and post new articles.");
}
