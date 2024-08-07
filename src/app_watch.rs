use chrono::{DateTime, Utc};
use log::{error, info};

use crate::{
    config::Config, feed_diff::FeedDiffManager, feed_fetch::FeedFetcher,
    feed_formatter::feed_format, feed_parse::feed_parse, misskey_post::MisskeyPost,
    schedules::Schedules, sleep::sleep_at,
};

pub async fn app_main(_args: &[String], config: &Config) {
    let mut feed_diff_manager = FeedDiffManager::new();
    let feed_fetcher = FeedFetcher::new(&config.feed_url);
    let misskey_post = MisskeyPost::new(&config.post_misskey_host, &config.post_misskey_api_token);

    let mut last_modified: Option<DateTime<Utc>> = None;

    let schedules = Schedules::parse(&config.cron).unwrap();

    info!("Starting…");

    loop {
        let fetch_result = feed_fetcher.fetch(last_modified).await;
        if let Some(fetch_result) = fetch_result {
            let feed = fetch_result.feed;
            let articles = feed_parse(feed);

            if let Some(articles) = articles {
                let new_articles =
                    feed_diff_manager.filter_new_feeds_and_update_last_time(&articles);

                for &v in new_articles.iter().rev() {
                    let message = feed_format(v);
                    let result = misskey_post.post(&message).await;

                    if let Err(err) = result {
                        error!("Error: {}", err);
                        error!("Failed to post: {}", message);
                    } else {
                        info!("{}", message);
                    }
                }
            }

            last_modified = fetch_result.last_modified;
        }

        info!("Fetched. {}", Utc::now());

        let next = schedules.upcoming_next().unwrap();
        sleep_at(next).await;
    }
}
