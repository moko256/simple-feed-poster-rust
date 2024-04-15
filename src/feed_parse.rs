use chrono::{DateTime, Utc};
use syndication::Feed;

pub struct FeedArticle {
    pub id: String,
    pub published: DateTime<Utc>,
    pub title: String,
    pub link: String,
}

pub fn feed_parse(parsed: Feed) -> Option<Vec<FeedArticle>> {
    match parsed {
        Feed::Atom(info) => {
            let articles = info.entries;
            let mut result: Vec<FeedArticle> = Vec::with_capacity(articles.len());

            for article in articles {
                result.push(FeedArticle {
                    id: article.id,
                    published: article.published?.to_utc(),
                    title: article.title.value,
                    link: article.links.first()?.href.clone(),
                })
            }

            Some(result)
        }
        Feed::RSS(info) => {
            let articles = info.items;
            let mut result: Vec<FeedArticle> = Vec::with_capacity(articles.len());

            for article in articles {
                result.push(FeedArticle {
                    id: article.guid?.value,
                    published: DateTime::parse_from_rfc2822(&article.pub_date?)
                        .ok()?
                        .to_utc(),
                    title: article.title?,
                    link: article.link?,
                })
            }

            Some(result)
        }
    }
}
