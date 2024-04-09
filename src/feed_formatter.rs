use std::{borrow::Borrow, collections::HashMap};

use strfmt::{strfmt, DisplayStr};
use syndication::Feed;

use crate::feed_parse::{CommonFeedEntries, CommonFeedInfo};

fn format_feed<'a, I, E>(feed_format: &str, feed_info: &'a I, feed_entry: &'a E)
where
    I: CommonFeedInfo<'a>,
    E: CommonFeedEntries<'a>,
{
    let args: HashMap<String, LazyDisplayStr> = HashMap::from([
        (
            "info_title".to_string(),
            LazyDisplayStr::new(|| feed_info.title().unwrap_or("")),
        ),
        (
            "info_generator".to_string(),
            LazyDisplayStr::new(|| feed_info.title().unwrap_or("")),
        ),
        (
            "info_rights".to_string(),
            LazyDisplayStr::new(|| feed_info.title().unwrap_or("")),
        ),
        (
            "info_links".to_string(),
            LazyDisplayStr::new(|| feed_info.title().unwrap_or("")),
        ),
        (
            "info_categories".to_string(),
            LazyDisplayStr::new(|| feed_info.title().unwrap_or("")),
        ),
        (
            "info_authors".to_string(),
            LazyDisplayStr::new(|| feed_info.title().unwrap_or("")),
        ),
    ]);

    let _ = strfmt("fmtstr", &args);
}

pub fn format_post_body<'a, I, E>(
    feed_format: &str,
    post_body_format: &str,
    feed_info: I,
    feed_entry: E,
) where
    I: CommonFeedInfo<'a>,
    E: CommonFeedEntries<'a>,
{
}

struct LazyDisplayStr<'a> {
    inner: Box<dyn Fn() -> &'a str>,
}

impl LazyDisplayStr<'_> {
    fn new<'a, F>(inner: F) -> LazyDisplayStr<'a>
    where
        F: 'a + Fn() -> &'a str,
    {
        LazyDisplayStr {
            inner: Box::new(inner),
        }
    }
}

impl<'a> DisplayStr for LazyDisplayStr<'a> {
    fn display_str(&self, f: &mut strfmt::Formatter) -> strfmt::Result<()> {
        (self.inner)().display_str(f)
    }
}
