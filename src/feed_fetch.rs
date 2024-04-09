use std::error::Error;

use chrono::{DateTime, Utc};
use reqwest::{header::HeaderMap, Client};
use syndication::Feed;

pub struct FeedFetcher<'a> {
    url: &'a str,
    client: Client,
}

impl FeedFetcher<'_> {
    pub fn new(url: &'_ str) -> FeedFetcher<'_> {
        let client = Client::builder()
            .user_agent(env!("CARGO_PKG_NAME"))
            .pool_max_idle_per_host(0)
            .tcp_keepalive(None)
            .build()
            .unwrap();

            FeedFetcher { url, client }
    }

    pub async fn fetch(self, last_modified: Option<&DateTime<Utc>>) -> Option<Feed> {
        let source = self.get_feed(last_modified).await;

        match source {
            Ok(source) => {
                let feed = source?.parse::<Feed>();

                match feed {
                    Ok(feed) => Some(feed),
                    Err(err) => {
                        print!("{}", err);

                        None
                    }
                }
            }
            Err(err) => {
                print!("{}", err);

                None
            }
        }
    }

    async fn get_feed(
        self,
        last_modified: Option<&DateTime<Utc>>,
    ) -> Result<Option<String>, impl Error> {
        let mut headers = HeaderMap::with_capacity(1);

        if let Some(last_modified) = last_modified {
            headers.insert(
                "If-Modified-Since",
                last_modified.to_rfc2822().parse().unwrap(),
            );
        }

        let result = self
            .client
            .get(self.url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        if result.status() == 304 {
            Ok(None)
        } else if result.status().is_success() {
            result.text().await.map(|x| Some(x))
        } else {
            Ok(None)
        }
    }
}
