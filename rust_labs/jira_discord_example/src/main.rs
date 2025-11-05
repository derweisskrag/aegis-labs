use reqwest::Client;
use serde_json::json;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    // let webhook_url = env::var("WEB_HOOK").expect("WEB_HOOK must be set");

    // let payload = json!({
    //     "content": "Hello from Rust! 🦀🚀",
    //     "username": "RustyBot",
    //     "avatar_url": "https://www.rust-lang.org/logos/rust-logo-512x512.png"
    // });

    // let client = Client::new();
    // let res = client.post(webhook_url)
    //     .json(&payload)
    //     .send()
    //     .await?;
    // let status = res.status();
    // println!("Status: {}", status);

    let auth_token = env::var("JIRA_TOKEN").expect("JIRA TOKEN must be set");
    let user_email = env::var("USER_EMAIL").expect("USER EMAIL must be set");
    let api_url = env::var("API_URL").expect("API URL must be set"); 
    let client = reqwest::Client::new();
    let res = client
        .get(api_url)
        .basic_auth(user_email, Some(auth_token))
        .send()
        .await?;

    let status = res.status();
    // let body = res.text().await?;
    println!("Status: {}", status);
    // println!("Body:\n{}", body);


    let json: serde_json::Value = res.json().await?;
    if let Some(issues) = json.get("issues").and_then(|v| v.as_array()) {
        for issue in issues {
            let key = issue.get("key").and_then(|v| v.as_str()).unwrap_or("N/A");
            let fields = issue.get("fields").unwrap_or(&serde_json::Value::Null);

            let summary = fields.get("summary").and_then(|v| v.as_str()).unwrap_or("No summary");
            let status = fields
                .get("status")
                .and_then(|s| s.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("No status");

            let description = fields.get("description").and_then(|v| v.as_str()).unwrap_or("No description");

            println!("{}: {}\nStatus: {}\nDescription: {}\n", key, summary, status, description);
        }
    } else {
        println!("No issues found or unexpected format.");
    }

    Ok(())
}
