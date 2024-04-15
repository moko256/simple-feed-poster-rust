use chrono::{DateTime, Utc};

use crate::feed_parse::FeedArticle;

pub struct FeedDiffManager {
    last_time: Option<DateTime<Utc>>,
}

impl FeedDiffManager {
    pub fn new() -> Self {
        FeedDiffManager { last_time: None }
    }

    pub fn filter_new_feeds_and_update_last_time<'a>(
        &mut self,
        feeds: &'a Vec<FeedArticle>,
    ) -> Vec<&'a FeedArticle> {
        let new_articles = if let Some(last_time) = self.last_time {
            let new_articles: Vec<&'a FeedArticle> =
                feeds.iter().filter(|a| a.published > last_time).collect();

            new_articles
        } else {
            // Retrieving is the first time. Treat as no new articles.
            Vec::with_capacity(0)
        };

        let latest_time = feeds.iter().max_by_key(|a| a.published);
        if let Some(latest_time) = latest_time {
            self.last_time = Some(latest_time.published);
        }

        new_articles
    }
}
