use super::{execute_twitter, TwitterResult};
use crate::fields::{
    media_fields::MediaFields, place_fields::PlaceFields, poll_fields::PollFields,
    tweet_fields::TweetFields, user_fields::UserFields,
};
use chrono::prelude::*;
use itertools::Itertools;
use reqwest::RequestBuilder;
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/tweets/search/stream";

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    AttachmentsPollIds,
    AttachmentsMediaKeys,
    AuthorId,
    EditHistoryTweetIds,
    EntitiesMentionsUsername,
    GeoPlaceId,
    InReplyToUserId,
    ReferencedTweetsId,
    ReferencedTweetsIdAuthorId,
}

impl Expansions {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::AttachmentsPollIds);
        result.insert(Self::AttachmentsMediaKeys);
        result.insert(Self::AuthorId);
        result.insert(Self::EditHistoryTweetIds);
        result.insert(Self::EntitiesMentionsUsername);
        result.insert(Self::GeoPlaceId);
        result.insert(Self::InReplyToUserId);
        result.insert(Self::ReferencedTweetsId);
        result.insert(Self::ReferencedTweetsIdAuthorId);
        result
    }
}

impl std::fmt::Display for Expansions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AttachmentsPollIds => write!(f, "attachments.poll_ids"),
            Self::AttachmentsMediaKeys => write!(f, "attachments.media_keys"),
            Self::AuthorId => write!(f, "author_id"),
            Self::EditHistoryTweetIds => write!(f, "edit_history_tweet_ids"),
            Self::EntitiesMentionsUsername => write!(f, "entities.mentions.username"),
            Self::GeoPlaceId => write!(f, "geo.place_id"),
            Self::InReplyToUserId => write!(f, "in_reply_to_user_id"),
            Self::ReferencedTweetsId => write!(f, "referenced_tweets.id"),
            Self::ReferencedTweetsIdAuthorId => write!(f, "referenced_tweets.id.author_id"),
        }
    }
}
#[derive(Debug, Default)]
pub struct Api {
    backfill_minutes: Option<usize>,
    end_time: Option<DateTime<Utc>>,
    expansions: Option<HashSet<Expansions>>,
    media_fields: Option<HashSet<MediaFields>>,
    place_fields: Option<HashSet<PlaceFields>>,
    poll_fields: Option<HashSet<PollFields>>,
    start_time: Option<DateTime<Utc>>,
    tweet_fields: Option<HashSet<TweetFields>>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn backfill_minutes(mut self, value: usize) -> Self {
        self.backfill_minutes = Some(value);
        self
    }

    pub fn end_time(mut self, value: DateTime<Utc>) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn expansions(mut self, value: HashSet<Expansions>) -> Self {
        self.expansions = Some(value);
        self
    }

    pub fn media_fields(mut self, value: HashSet<MediaFields>) -> Self {
        self.media_fields = Some(value);
        self
    }

    pub fn place_fields(mut self, value: HashSet<PlaceFields>) -> Self {
        self.place_fields = Some(value);
        self
    }

    pub fn poll_fields(mut self, value: HashSet<PollFields>) -> Self {
        self.poll_fields = Some(value);
        self
    }

    pub fn start_time(mut self, value: DateTime<Utc>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn tweet_fields(mut self, value: HashSet<TweetFields>) -> Self {
        self.tweet_fields = Some(value);
        self
    }

    pub fn user_fields(mut self, value: HashSet<UserFields>) -> Self {
        self.user_fields = Some(value);
        self
    }

    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(backfill_minutes) = self.backfill_minutes {
            query_parameters.push(("backfill_minutes", backfill_minutes.to_string()));
        }
        if let Some(end_time) = self.end_time {
            query_parameters.push(("end_time", end_time.format("%Y-%m-%dT%H%M%SZ").to_string()));
        }
        if let Some(expansions) = self.expansions {
            query_parameters.push(("expansions", expansions.iter().join(",")));
        }
        if let Some(media_fields) = self.media_fields {
            query_parameters.push(("media_fields", media_fields.iter().join(",")));
        }
        if let Some(place_fields) = self.place_fields {
            query_parameters.push(("place_fields", place_fields.iter().join(",")));
        }
        if let Some(poll_fields) = self.poll_fields {
            query_parameters.push(("poll_fields", poll_fields.iter().join(",")));
        }
        if let Some(start_time) = self.start_time {
            query_parameters.push((
                "start_time",
                start_time.format("%Y-%m-%dT%H%M%SZ").to_string(),
            ));
        }
        if let Some(tweet_fields) = self.tweet_fields {
            query_parameters.push(("tweet_fields", tweet_fields.iter().join(",")));
        }
        if let Some(user_fields) = self.user_fields {
            query_parameters.push(("user_fields", user_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        client
            .get(URL)
            .query(&query_parameters)
            .bearer_auth(bearer_code)
    }

    pub async fn execute(self, bearer_code: &str) -> TwitterResult {
        execute_twitter(self.build(bearer_code)).await
    }
}