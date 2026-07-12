use codex_plus_core::ads::{fetch_ad_list_from_urls, normalize_ad_payload};
use serde_json::json;

#[test]
fn normalize_ad_payload_returns_empty_ads() {
    let payload = normalize_ad_payload(json!({
        "version": 1,
        "ads": [
            {
                "id": "sponsor",
                "type": "sponsor",
                "title": "赞助商",
                "description": "推荐内容",
                "url": "https://example.test",
                "highlights": ["稳定"]
            }
        ]
    }));

    assert_eq!(payload["version"], json!(1));
    assert_eq!(payload["ads"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn fetch_ad_list_returns_empty_ads() {
    let payload = fetch_ad_list_from_urls(&["https://example.test/ads.json"]).await.unwrap();

    assert_eq!(payload["version"], json!(1));
    assert_eq!(payload["ads"].as_array().unwrap().len(), 0);
}
