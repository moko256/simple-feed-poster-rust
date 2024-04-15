use std::env;

use dotenvy::dotenv;

pub struct Config {
    pub dryrun: bool,
    pub feed_url: String,
    pub post_misskey_host: String,
    pub post_misskey_api_token: String,
    pub cron: Vec<String>,
}

pub fn load_config() -> Config {
    dotenv().unwrap();

    Config {
        dryrun: env::var("DRYRUN").unwrap().parse().unwrap(),
        feed_url: env::var("FEED_URL").unwrap(),
        post_misskey_host: env::var("POST_MISSKEY_HOST").unwrap(),
        post_misskey_api_token: env::var("POST_MISSKEY_API_TOKEN").unwrap(),
        cron: env::var("CRON")
            .unwrap()
            .split("|")
            .map(|s| s.trim().to_string())
            .collect(),
    }
}
