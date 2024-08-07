use std::error::Error;

use chrono::{DateTime, Utc};
use log::warn;
use reqwest::{header::HeaderMap, Client, Response};
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

    pub async fn fetch(
        &self,
        last_modified: Option<DateTime<Utc>>,
    ) -> Option<FeedFetcherFetchResult> {
        let source = self.get_feed(last_modified).await;

        match source {
            Ok(source) => {
                let source = source?;
                let feed = source.body.parse::<Feed>();

                let new_last_modified = source.last_modified;

                match feed {
                    Ok(feed) => Some(FeedFetcherFetchResult {
                        feed: feed,
                        last_modified: new_last_modified,
                    }),
                    Err(err) => {
                        warn!("{}", err);

                        None
                    }
                }
            }
            Err(err) => {
                warn!("{}", err);

                None
            }
        }
    }

    async fn get_feed(
        &self,
        last_modified: Option<DateTime<Utc>>,
    ) -> Result<Option<HttpFetchResult>, impl Error> {
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

        let new_last_modified = get_last_modified(&result);

        if result.status() == 304 {
            Ok(None)
        } else if result.status().is_success() {
            result.text().await.map(|x| {
                Some(HttpFetchResult {
                    body: x,
                    last_modified: new_last_modified,
                })
            })
        } else {
            Ok(None)
        }
    }
}

fn get_last_modified(response: &Response) -> Option<DateTime<Utc>> {
    Some(
        DateTime::parse_from_rfc2822(&response.headers().get("Last-Modified")?.to_str().ok()?)
            .ok()?
            .to_utc(),
    )
}

pub struct FeedFetcherFetchResult {
    pub feed: Feed,
    pub last_modified: Option<DateTime<Utc>>,
}

struct HttpFetchResult {
    body: String,
    last_modified: Option<DateTime<Utc>>,
}
