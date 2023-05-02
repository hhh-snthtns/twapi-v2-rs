use anyhow::Result;
use twapi_v2::api::{execute_twitter, post_2_tweets_search_stream_rules};

// BEARER_CODE=XXXXX cargo test test_post_2_tweets_search_stream_rules -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_tweets_search_stream_rules() -> Result<()> {
    let body = post_2_tweets_search_stream_rules::Body {
        add: Some(vec![
            post_2_tweets_search_stream_rules::Add {
                value: "東京".to_owned(),
                ..Default::default()
            },
            post_2_tweets_search_stream_rules::Add {
                value: "大阪".to_owned(),
                ..Default::default()
            },
        ]),
        ..Default::default()
    };
    /*
    let body = post_2_tweets_search_stream_rules::Body {
        delete: Some(post_2_tweets_search_stream_rules::Delete {
                ids: vec![
                    "xxxxx".to_owned(),
                    "xxxxx".to_owned(),
                ],
                ..Default::default()
            }
        ),
        ..Default::default()
    };
    */
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = post_2_tweets_search_stream_rules::Api::new(&bearer_code, body).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_tweets_search_stream_rules::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
