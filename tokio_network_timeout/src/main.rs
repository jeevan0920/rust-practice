use actix_web::{get, App, HttpServer, Responder};
use log::{error, info};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Serialize, Deserialize, Debug)]
struct CustomerConfig {
    customer_key: String,
    customer_id: String,
    obfuscate_ip: String,
    rules: Vec<Rule>,
    meta_map: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rule {
    ids: Vec<String>,
    match_key: Vec<String>,
    pii_fields: Vec<String>,
}

async fn fetch_configs_from_customer_keys(
    url: &str,
    namespace: &str,
    customer_keys: &[&str],
) -> Result<HashMap<String, CustomerConfig>, ()> {
    let query = json!({
        "query": {
            "namespace_id": namespace,
            "filter": {
                "scopes": customer_keys.iter().map(|key| json!({"name": "customer_key", "value": key})).collect::<Vec<_>>(),
            }
        }
    });

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = Client::new();
    let response = match client
        .post(url)
        .json(&query)
        .headers(headers.clone())
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to send request: {}", e);
            return Err(());
        }
    };

    let status = response.status();
    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            error!("Failed to read response body: {}", e);
            return Err(());
        }
    };

    if status == reqwest::StatusCode::OK {
        let data: Value = match serde_json::from_str(&body) {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to parse JSON: {}", e);
                return Err(());
            }
        };
        let mut results = HashMap::new();
        if let Some(customer_configs) = data["data"].as_array() {
            for config in customer_configs {
                let customer_key = config["scope"]["value"].as_str().unwrap().to_string();
                let customer_config: CustomerConfig =
                    match serde_json::from_value(config["config"].clone()) {
                        Ok(config) => config,
                        Err(e) => {
                            error!("Failed to parse customer config: {}", e);
                            return Err(());
                        }
                    };
                results.insert(customer_key, customer_config);
            }
        }
        Ok(results)
    } else {
        error!(
            "FetchConfig request failed - Headers: {:?}, Query: {:?}, Response Status: {}, Response Body: '{}'",
            headers, query, status, body
        );
        Err(())
    }
}

#[get("/fetch")]
async fn fetch_handler() -> impl Responder {
    let namespace = env::var("CCS_NAMESPACE").expect("CCS_NAMESPACE not set");
    let ccs_endpoint = env::var("CCS_ENDPOINT").expect("CCS_ENDPOINT not set");
    let url = format!("{}/v1/config/get?x-trace-id=example_trace_id", ccs_endpoint);
    let customer_keys = vec!["customer_key1", "customer_key2"]; // Replace with actual customer keys

    let result = timeout(
        Duration::from_secs(5),
        fetch_configs_from_customer_keys(&url, &namespace, &customer_keys),
    )
    .await;

    match result {
        Ok(Ok(configs)) => {
            info!("Successfully fetched configs");
            format!("Configs: {:?}", configs)
        }
        Ok(Err(e)) => {
            error!("Request error: {:?}", e);
            format!("Request error: {:?}", e)
        }
        Err(e) => {
            error!("Timeout error: {:?}", e);
            "Timeout error".to_string()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| App::new().service(fetch_handler))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
