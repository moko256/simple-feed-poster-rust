use crate::feed_parse::FeedArticle;

pub fn feed_format(parsed: &FeedArticle) -> String {
    return format!("{}\n\n{}", parsed.title, parsed.link);
}
