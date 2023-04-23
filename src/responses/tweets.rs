use crate::responses::{
    attachments::Attachments, context_annotations::ContextAnnotations, edit_controls::EditControls,
    entities::Entities, geo::Geo, public_metrics::PublicMetrics,
    referenced_tweets::ReferencedTweets,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tweets {
    pub attachments: Option<Attachments>,
    pub author_id: Option<String>,
    pub context_annotations: Option<Vec<ContextAnnotations>>,
    pub conversation_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub edit_controls: Option<EditControls>,
    pub edit_history_tweet_ids: Vec<String>,
    pub entities: Option<Entities>,
    pub geo: Option<Geo>,
    pub id: String,
    pub in_reply_to_user_id: Option<String>,
    pub lang: Option<String>,
    pub possibly_sensitive: Option<bool>,
    pub public_metrics: Option<PublicMetrics>,
    pub reply_settings: Option<String>,
    pub referenced_tweets: Option<Vec<ReferencedTweets>>,
    pub text: String,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
