use std::borrow::Borrow;

use chrono::{DateTime, Utc};

// Use atom name
pub trait CommonFeedInfo<'a> {
    fn title(&'a self) -> Option<&'a str>;
    fn generator(&'a self) -> Option<&'a str>;
    fn rights(&'a self) -> Option<&'a str>;
    fn links(&'a self) -> Option<Vec<&'a str>>;
    fn categories(&'a self) -> Option<Vec<&'a str>>;
    fn authors(&'a self) -> Option<Vec<&'a str>>;
}

pub trait CommonFeedEntries<'a> {
    fn id(&'a self) -> Option<&'a str>;
    fn title(&'a self) -> Option<&'a str>;
    fn published(&'a self) -> Option<DateTime<Utc>>; // Date
    fn links(&'a self) -> Option<Vec<&'a str>>;
    fn categories(&'a self) -> Option<Vec<&'a str>>;
    fn summary(&'a self) -> Option<&'a str>;
    fn content(&'a self) -> Option<&'a str>;
}

#[derive(Debug)]
pub struct AtomFeedInfo<'a>(&'a atom_syndication::Feed);

#[derive(Debug)]
pub struct AtomFeedEntries<'a>(&'a atom_syndication::Entry);

#[derive(Debug)]
pub struct RssFeedInfo<'a>(&'a rss::Channel);

#[derive(Debug)]
pub struct RssFeedEntries<'a>(&'a rss::Item);

impl<'a> AtomFeedInfo<'a> {
    pub fn new(n: &'a atom_syndication::Feed) -> Self {
        Self(n)
    }
}

impl<'a> AtomFeedEntries<'a> {
    pub fn new(n: &'a atom_syndication::Entry) -> Self {
        Self(n)
    }
}

impl<'a> RssFeedInfo<'a> {
    pub fn new(n: &'a rss::Channel) -> Self {
        Self(n)
    }
}

impl<'a> RssFeedEntries<'a> {
    pub fn new(n: &'a rss::Item) -> Self {
        Self(n)
    }
}

impl<'a> CommonFeedInfo<'a> for AtomFeedInfo<'_> {
    fn title(&'a self) -> Option<&'a str> {
        Some(&self.0.title().value)
    }

    fn generator(&'a self) -> Option<&'a str> {
        Some(&self.0.generator()?.value)
    }

    fn rights(&'a self) -> Option<&'a str> {
        Some(&self.0.rights()?.value)
    }

    fn links(&'a self) -> Option<Vec<&'a str>> {
        Some(self.0.links.iter().map(|x| &*x.href).collect::<Vec<&str>>())
    }

    fn categories(&'a self) -> Option<Vec<&'a str>> {
        Some(
            self.0
                .categories
                .iter()
                .map(|x| &*x.term)
                .collect::<Vec<&str>>(),
        )
    }

    fn authors(&'a self) -> Option<Vec<&'a str>> {
        Some(
            self.0
                .authors
                .iter()
                .map(|x| &*x.name)
                .collect::<Vec<&str>>(),
        )
    }
}

impl<'a> CommonFeedEntries<'a> for AtomFeedEntries<'_> {
    fn id(&'a self) -> Option<&'a str> {
        Some(&self.0.id())
    }

    fn title(&'a self) -> Option<&'a str> {
        Some(&self.0.title().value)
    }

    fn published(&'a self) -> Option<DateTime<Utc>> {
        self.0.published().map(|x| x.to_utc())
    }

    fn links(&'a self) -> Option<Vec<&'a str>> {
        Some(self.0.links.iter().map(|x| &*x.href).collect::<Vec<&str>>())
    }

    fn categories(&'a self) -> Option<Vec<&'a str>> {
        Some(
            self.0
                .categories
                .iter()
                .map(|x| &*x.term)
                .collect::<Vec<&str>>(),
        )
    }

    fn summary(&'a self) -> Option<&'a str> {
        Some(&self.0.summary()?.value)
    }

    fn content(&'a self) -> Option<&'a str> {
        Some(&self.0.content()?.value.as_ref()?)
    }
}

impl<'a> CommonFeedInfo<'a> for RssFeedInfo<'_> {
    fn title(&'a self) -> Option<&'a str> {
        Some(self.0.title())
    }

    fn generator(&'a self) -> Option<&'a str> {
        self.0.generator()
    }

    fn rights(&'a self) -> Option<&'a str> {
        self.0.copyright()
    }

    fn links(&'a self) -> Option<Vec<&'a str>> {
        Some(Vec::from([self.0.link()]))
    }

    fn categories(&'a self) -> Option<Vec<&'a str>> {
        Some(
            self.0
                .categories()
                .iter()
                .map(|x| &*x.name)
                .collect::<Vec<&str>>(),
        )
    }

    fn authors(&'a self) -> Option<Vec<&'a str>> {
        None
    }
}

impl<'a> CommonFeedEntries<'a> for RssFeedEntries<'_> {
    fn id(&'a self) -> Option<&'a str> {
        Some(&*self.0.guid()?.value)
    }

    fn title(&'a self) -> Option<&'a str> {
        self.0.title()
    }

    fn published(&'a self) -> Option<DateTime<Utc>> {
        Some(
            DateTime::parse_from_rfc2822(self.0.pub_date()?)
                .ok()?
                .to_utc(),
        )
    }

    fn links(&'a self) -> Option<Vec<&'a str>> {
        Some(Vec::from([self.0.link()?]))
    }

    fn categories(&'a self) -> Option<Vec<&'a str>> {
        Some(
            self.0
                .categories()
                .iter()
                .map(|x| &*x.name)
                .collect::<Vec<&str>>(),
        )
    }

    fn summary(&'a self) -> Option<&'a str> {
        self.0.description()
    }

    fn content(&'a self) -> Option<&'a str> {
        self.0.content()
    }
}
