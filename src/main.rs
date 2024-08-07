mod app_misskey_post;
mod app_show;
mod app_watch;
mod config;
mod feed_diff;
mod feed_fetch;
mod feed_formatter;
mod feed_parse;
mod misskey_post;
mod schedules;
mod sleep;

use std::env;

use moko256_systemd_stdio_logger as logger;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    logger::init([
        logger::LoggerModuleFilterKey::Module(module_path!(), log::LevelFilter::Info),
        logger::LoggerModuleFilterKey::Default(log::LevelFilter::Warn),
    ])
    .unwrap();

    let args: Vec<String> = env::args().collect();

    let command = args.get(1);

    let config = config::load_config();

    match command {
        Some(command) => match command.as_str() {
            "watch" => {
                app_watch::app_main(&args, &config).await;
            }
            "show" => {
                app_show::app_main(&args, &config).await;
            }
            "post" => {
                app_misskey_post::app_main(&args, &config).await;
            }
            _ => {
                print_help();
            }
        },
        None => {
            print_help();
        }
    }
}

fn print_help() {
    println!("{}", env!("CARGO_PKG_NAME"));
    println!("");
    println!("Commands:");
    println!("  show      Show all feed.");
    println!("  watch     Start watching feed and post new articles.");
    println!("  post      Post to misskey.");
}
