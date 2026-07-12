use serde_json::{Value, json};

pub fn normalize_ad_payload(payload: Value) -> Value {
    let version = payload.get("version").and_then(Value::as_u64).unwrap_or(1);
    json!({ "version": version, "ads": [] })
}

pub async fn fetch_ad_list() -> anyhow::Result<Value> {
    Ok(json!({ "version": 1, "ads": [] }))
}

pub async fn fetch_ad_list_from_urls<S>(_urls: &[S]) -> anyhow::Result<Value>
where
    S: AsRef<str>,
{
    Ok(json!({ "version": 1, "ads": [] }))
}
