use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_lists_id_tweets};
mod common;

// BEARER_CODE=XXXXX cargo test test_get_2_lists_id_tweets -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_lists_id_tweets() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = get_2_lists_id_tweets::Api::open(&bearer_code, "14914404").build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    common::write_json(&res, "get_2_lists_id_tweets")?;
    let response = serde_json::from_value::<get_2_lists_id_tweets::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
