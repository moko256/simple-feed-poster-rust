use std::time::Duration;

use chrono::Local;
use tokio::time::sleep;

use crate::{
    config::Config, feed_diff::FeedDiffManager, feed_fetch::FeedFetcher,
    feed_formatter::feed_format, feed_parse::feed_parse, schedules::Schedules,
};

pub async fn app_main(_args: &[String], config: &Config) {
    let mut diff = FeedDiffManager::new();

    let fetcher = FeedFetcher::new(&config.feed_url);
    let feed = fetcher.fetch(None).await.unwrap();

    let articles = feed_parse(feed.feed).unwrap();

    diff.filter_new_feeds_and_update_last_time(&articles);

    for article in articles {
        println!(
            "Date: {}\nID: {}\n\n{}\n\n====",
            article.published.with_timezone(&Local).to_rfc3339(),
            article.id,
            feed_format(&article)
        )
    }

    let last_modified = feed.last_modified;
    println!("Last Modified: {:?}", last_modified);

    let schedules = Schedules::parse(&config.cron).unwrap();
    let next_time = schedules.upcoming_next().unwrap();
    println!("\nCron Next Time: {}", next_time.to_rfc3339());

    sleep(Duration::from_secs(1)).await;

    let feed2 = fetcher.fetch(last_modified).await;
    if let Some(feed2) = feed2 {
        let article2 = feed_parse(feed2.feed).unwrap();
        let new_articles = diff.filter_new_feeds_and_update_last_time(&article2);
        let new_last_modified = feed2.last_modified;
        println!("Retry: Content is modified. Length: {}", new_articles.len());
        println!("New Last Modified: {:?}", new_last_modified);
    } else {
        println!("Retry: Content isn't modified.");
    }
}
