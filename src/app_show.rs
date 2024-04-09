use crate::{
    config::Config,
    feed_fetch::{self, FeedFetcher},
    feed_parse::{CommonFeedInfo, RssFeedEntries, RssFeedInfo},
};

pub async fn app_main(_args: &[String], config: &Config) {
    let fetcher = FeedFetcher::new(&config.post_url);
    let feed = fetcher.fetch(None).await.unwrap();

    match feed {
        syndication::Feed::Atom(feed) => todo!(),
        syndication::Feed::RSS(info) => {
            for entry in &info.items {
                println!(
                    "{:?} {:?}",
                    RssFeedInfo::new(&info),
                    RssFeedEntries::new(&entry)
                )
            }
        }
    }
}
