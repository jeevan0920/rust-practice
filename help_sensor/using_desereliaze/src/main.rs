use serde::{Deserialize, Serialize};
use serde_json::Value;

// Define the structure for the "body" field
#[derive(Debug, Serialize, Deserialize)]
struct Body {
    schema: String,
    data: Vec<CollectorPayload>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct CollectorPayload {
  pub ip_address: Option<String>,
  pub timestamp: Option<i64>,
  pub encoding: Option<String>,
  pub collector: Option<String>,
  pub user_agent: Option<String>,
  pub referer_uri: Option<String>,
  pub path: Option<String>,
  pub querystring: Option<String>,
  pub body: Option<String>,
  pub headers: Option<Vec<String>>,
  pub content_type: Option<String>,
  pub hostname: Option<String>,
  pub network_user_id: Option<String>,
  pub schema: Option<String>,
}

fn main() {
    // Your log line
    // let log_line = r#""2023-12-14 14:53:47.557 - [1;34mINFO[0m [gateway::handlers::event] - Universal Identity Headers: HeaderMap { inner: {...}, Receive body size: 3687, Receive data: b"\"{\\\"schema\\\":...}\", Ip address: \"152.57.209.246\" - src/handlers/event.rs:111""#;
    // let log_line = r#""2023-12-14 14:53:47.557 - [1;34mINFO[0m [gateway::handlers::event] - Universal Identity Headers: HeaderMap { inner: {""accept-encoding"": Value { inner: [""gzip""] }, ""via"": Value { inner: [""1.1 google""] }, ""x-scheme"": Value { inner: [""https""] }, ""x-client-id"": Value { inner: [""1101591265.2050112126.2592844066.3048833603""] }, ""x-forwarded-scheme"": Value { inner: [""https""] }, ""x-original-forwarded-for"": Value { inner: [""152.57.209.246,34.36.74.50""] }, ""x-cloud-trace-context"": Value { inner: [""cc3305e72b3d6a26438a238d37aa4fad/10174312007067708867""] }, ""x-request-id"": Value { inner: [""ac08b4dc6b7e9806a92c1d51d5201999""] }, ""host"": Value { inner: [""appgw.conviva.com""] }, ""x-forwarded-for"": Value { inner: [""152.57.209.246,34.36.74.50, 35.191.17.132"", ""152.57.209.246,34.36.74.50, 152.57.209.246""] }, ""x-forwarded-host"": Value { inner: [""appgw.conviva.com""] }, ""x-real-ip"": Value { inner: [""152.57.209.246""] }, ""x-forwarded-proto"": Value { inner: [""https""] }, ""content-type"": Value { inner: [""application/json; charset=utf-8""] }, ""content-length"": Value { inner: [""3687""] }, ""x-forwarded-port"": Value { inner: [""80""] }, ""user-agent"": Value { inner: [""Dalvik/2.1.0 (Linux; U; Android 12; moto g22 Build/STAS32.79-77-28-50-7)""] }} }, Receive body size: 3687, Receive data: b""{\""schema\"":\""iglu:com.snowplowanalytics.snowplow\\/payload_data\\/jsonschema\\/1-0-4\"",\""data\"":[{\""eid\"":\""032e6af7-d1d8-4f1e-bf75-820b86ba11cb\"",\""res\"":\""720x1440\"",\""tv\"":\""andr-0.7.5.1\"",\""e\"":\""ue\"",\""tna\"":\""CAT\"",\""tz\"":\""Asia\\/Kolkata\"",\""stm\"":\""1702565625727\"",\""p\"":\""mob\"",\""uid\"":\""Z5X_7de0c90893187ebc362f6bf5dc8e41d4b7d46ba37b8b4c2fbd232f07ea9a39e1\"",\""cx\"":\""eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvY29udGV4dHNcL2pzb25zY2hlbWFcLzEtMC0xIiwiZGF0YSI6W3sic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3Muc25vd3Bsb3dcL2NsaWVudF9zZXNzaW9uXC9qc29uc2NoZW1hXC8xLTAtMiIsImRhdGEiOnsic2Vzc2lvbkluZGV4IjoyLCJzdG9yYWdlTWVjaGFuaXNtIjoiTE9DQUxfU1RPUkFHRSIsImZpcnN0RXZlbnRUaW1lc3RhbXAiOiIyMDIzLTEyLTEwVDExOjEyOjQyLjA3OFoiLCJmaXJzdEV2ZW50SWQiOiJkZDU0NDFhZi00MWM5LTQzY2EtYjUxZi0yN2Q2ZjBkM2MzYWEiLCJzZXNzaW9uSWQiOiIyZmI1YmE1MC0zNDcxLTQxMGYtYjQ3Ni1hOWE5Y2Y0NTNjZWMiLCJldmVudEluZGV4IjoyNSwicHJldmlvdXNTZXNzaW9uSWQiOiI4MzM5YzQ5MS1kZjEyLTRkY2YtOWM5MC0xMTBhMGEyNjU5NDUiLCJ1c2VySWQiOiIzODkyMWFiZC1kNGZmLTQ2MmUtODUyNi1kMmRmYzIzYTU2YzIifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5tb2JpbGVcL2FwcGxpY2F0aW9uXC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsiYnVpbGQiOiIyMDMzMTE2NzgiLCJ2ZXJzaW9uIjoiMzguNzEuMTAifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvbW9iaWxlX2NvbnRleHRcL2pzb25zY2hlbWFcLzEtMC0yIiwiZGF0YSI6eyJjYXJyaWVyIjoiSklPIDRHIiwidG90YWxTdG9yYWdlIjo1NTQ4NTM2NjI3Miwic3lzdGVtQXZhaWxhYmxlTWVtb3J5IjoxNzY4NjMyMzIwLCJvc1ZlcnNpb24iOiIxMiIsImJhdHRlcnlTdGF0ZSI6InVucGx1Z2dlZCIsImF2YWlsYWJsZVN0b3JhZ2UiOjI0NzkyNjg2NTkyLCJvc1R5cGUiOiJhbmRyb2lkIiwiYW5kcm9pZElkZmEiOiJkMGEyMmMxZC0wZDNkLTRiYWEtYjBlYy1hNTk1ZjY4N2M5ZjIiLCJkZXZpY2VNb2RlbCI6Im1vdG8gZzIyIiwibG93UG93ZXJNb2RlIjpmYWxzZSwiZGV2aWNlTWFudWZhY3R1cmVyIjoibW90b3JvbGEiLCJuZXR3b3JrVHlwZSI6Im9mZmxpbmUiLCJwaHlzaWNhbE1lbW9yeSI6Mzk1MzI2NjY4OCwiYmF0dGVyeUxldmVsIjo1N319LHsic2NoZW1hIjoiaWdsdTpjb20uY29udml2YVwvY2xpZF9zY2hlbWFcL2pzb25zY2hlbWFcLzEtMC0yIiwiZGF0YSI6eyJpaWQiOiI4MTk0NzE1ODMiLCJjayI6IjI4YmJmMGEzZGVjZjZkMTE2NTAzMmJmZDgzNWQ2ZTE4NDRjNWQ0NGQiLCJjbGlkIjoiMTEwMTU5MTI2NS4yMDUwMTEyMTI2LjI1OTI4NDQwNjYuMzA0ODgzMzYwMyIsImV2ZW50SW5kZXgiOjY2fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhLm1vYmlsZVwvYW5kcm9pZF9hcHBfbG9hZHRpbWVcL2pzb25zY2hlbWFcLzEtMC0xIiwiZGF0YSI6eyJvbkNyZWF0ZSI6MTcwMjIwNjc2MTgzNiwiYXBwTG9hZCI6MTcwMjIwNjc2MTAxOSwib25SZXN1bWUiOjE3MDIyMDY3NjIxNDV9fSx7InNjaGVtYSI6ImlnbHU6Y29tLmNvbnZpdmFcL2V2ZW50X2luZm9cL2pzb25zY2hlbWFcLzEtMC0xIiwiZGF0YSI6eyJwcmV2aW91c0V2ZW50VGltZXN0YW1wIjoiMjAyMy0xMi0xMFQxMTozMzoyMi43MTVaIiwiZXZlbnRJbmRleCI6MjV9fSx7InNjaGVtYSI6ImlnbHU6Y29tLmNvbnZpdmFcL2N1c3RvbV90YWdzXC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsiZGF0YSI6e319fSx7InNjaGVtYSI6ImlnbHU6Y29tLnNub3dwbG93YW5hbHl0aWNzLm1vYmlsZVwvc2NyZWVuXC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsiYWN0aXZpdHkiOiJjb20uemVlNS5NYWluQWN0aXZpdHkiLCJuYW1lIjoiY29tLnplZTUuTWFpbkFjdGl2aXR5IiwiaWQiOiI2ZjFkNmE2NC0yYmZjLTQ3NmQtOWRlZC02ZjQyYzkzY2NhNDEiLCJ0eXBlIjoiY29tLnplZTUuTWFpbkFjdGl2aXR5In19LHsic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3MubW9iaWxlXC9hcHBsaWNhdGlvbl9saWZlY3ljbGVcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJpbmRleCI6MSwiaXNWaXNpYmxlIjp0cnVlfX1dfQ==\"",\""ue_px\"":\""eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvdW5zdHJ1Y3RfZXZlbnRcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5tb2JpbGVcL3NjcmVlbl92aWV3XC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsibmFtZSI6ImNvbS56ZWU1Lk1haW5BY3Rpdml0eSIsInByZXZpb3VzSWQiOiJkYjY0NzBkZi0xNDdiLTRjMjYtYjQyMi0wNDYxODdmNDdkNWMiLCJpZCI6IjZmMWQ2YTY0LTJiZmMtNDc2ZC05ZGVkLTZmNDJjOTNjY2E0MSIsInR5cGUiOiJjb20uemVlNS5NYWluQWN0aXZpdHkiLCJwcmV2aW91c05hbWUiOiJjb20uemVlNS5NYWluQWN0aXZpdHkiLCJwcmV2aW91c1R5cGUiOiJjb20uemVlNS5NYWluQWN0aXZpdHkifX19\"",\""dtm\"":\""1702208002729\"",\""lang\"":\""English\"",\""aid\"":\""Android-Mobile-Zee5\""}]}"", Ip address: ""152.57.209.246"" - src/handlers/event.rs:111""#;

    let log_line = r#""2023-12-14 14:53:47.557 - [1;34mINFO[0m [gateway::handlers::event] - Universal Identity Headers: HeaderMap { inner: {""accept-encoding"": Value { inner: [""gzip""] }, ""via"": Value { inner: [""1.1 google""] }, ""x-scheme"": Value { inner: [""https""] }, ""x-client-id"": Value { inner: [""1101591265.2050112126.2592844066.3048833603""] }, ""x-forwarded-scheme"": Value { inner: [""https""] }, ""x-original-forwarded-for"": Value { inner: [""152.57.209.246,34.36.74.50""] }, ""x-cloud-trace-context"": Value { inner: [""cc3305e72b3d6a26438a238d37aa4fad/10174312007067708867""] }, ""x-request-id"": Value { inner: [""ac08b4dc6b7e9806a92c1d51d5201999""] }, ""host"": Value { inner: [""appgw.conviva.com""] }, ""x-forwarded-for"": Value { inner: [""152.57.209.246,34.36.74.50, 35.191.17.132"", ""152.57.209.246,34.36.74.50, 152.57.209.246""] }, ""x-forwarded-host"": Value { inner: [""appgw.conviva.com""] }, ""x-real-ip"": Value { inner: [""152.57.209.246""] }, ""x-forwarded-proto"": Value { inner: [""https""] }, ""content-type"": Value { inner: [""application/json; charset=utf-8""] }, ""content-length"": Value { inner: [""3687""] }, ""x-forwarded-port"": Value { inner: [""80""] }, ""user-agent"": Value { inner: [""Dalvik/2.1.0 (Linux; U; Android 12; moto g22 Build/STAS32.79-77-28-50-7)""] }} }, Receive body size: 3687, Receive data: b""{\""schema\"":\""iglu:com.snowplowanalytics.snowplow\\/payload_data\\/jsonschema\\/1-0-4\"",\""data\"":[{\""eid\"":\""032e6af7-d1d8-4f1e-bf75-820b86ba11cb\"",\""res\"":\""720x1440\"",\""tv\"":\""andr-0.7.5.1\"",\""e\"":\""ue\"",\""tna\"":\""CAT\"",\""tz\"":\""Asia\\/Kolkata\"",\""stm\"":\""1702565625727\"",\""p\"":\""mob\"",\""uid\"":\""Z5X_7de0c90893187ebc362f6bf5dc8e41d4b7d46ba37b8b4c2fbd232f07ea9a39e1\"",\""cx\"":\""eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvY29udGV4dHNcL2pzb25zY2hlbWFcLzEtMC0xIiwiZGF0YSI6W3sic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3Muc25vd3Bsb3dcL2NsaWVudF9zZXNzaW9uXC9qc29uc2NoZW1hXC8xLTAtMiIsImRhdGEiOnsic2Vzc2lvbkluZGV4IjoyLCJzdG9yYWdlTWVjaGFuaXNtIjoiTE9DQUxfU1RPUkFHRSIsImZpcnN0RXZlbnRUaW1lc3RhbXAiOiIyMDIzLTEyLTEwVDExOjEyOjQyLjA3OFoiLCJmaXJzdEV2ZW50SWQiOiJkZDU0NDFhZi00MWM5LTQzY2EtYjUxZi0yN2Q2ZjBkM2MzYWEiLCJzZXNzaW9uSWQiOiIyZmI1YmE1MC0zNDcxLTQxMGYtYjQ3Ni1hOWE5Y2Y0NTNjZWMiLCJldmVudEluZGV4IjoyNSwicHJldmlvdXNTZXNzaW9uSWQiOiI4MzM5YzQ5MS1kZjEyLTRkY2YtOWM5MC0xMTBhMGEyNjU5NDUiLCJ1c2VySWQiOiIzODkyMWFiZC1kNGZmLTQ2MmUtODUyNi1kMmRmYzIzYTU2YzIifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5tb2JpbGVcL2FwcGxpY2F0aW9uXC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsiYnVpbGQiOiIyMDMzMTE2NzgiLCJ2ZXJzaW9uIjoiMzguNzEuMTAifX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvbW9iaWxlX2NvbnRleHRcL2pzb25zY2hlbWFcLzEtMC0yIiwiZGF0YSI6eyJjYXJyaWVyIjoiSklPIDRHIiwidG90YWxTdG9yYWdlIjo1NTQ4NTM2NjI3Miwic3lzdGVtQXZhaWxhYmxlTWVtb3J5IjoxNzY4NjMyMzIwLCJvc1ZlcnNpb24iOiIxMiIsImJhdHRlcnlTdGF0ZSI6InVucGx1Z2dlZCIsImF2YWlsYWJsZVN0b3JhZ2UiOjI0NzkyNjg2NTkyLCJvc1R5cGUiOiJhbmRyb2lkIiwiYW5kcm9pZElkZmEiOiJkMGEyMmMxZC0wZDNkLTRiYWEtYjBlYy1hNTk1ZjY4N2M5ZjIiLCJkZXZpY2VNb2RlbCI6Im1vdG8gZzIyIiwibG93UG93ZXJNb2RlIjpmYWxzZSwiZGV2aWNlTWFudWZhY3R1cmVyIjoibW90b3JvbGEiLCJuZXR3b3JrVHlwZSI6Im9mZmxpbmUiLCJwaHlzaWNhbE1lbW9yeSI6Mzk1MzI2NjY4OCwiYmF0dGVyeUxldmVsIjo1N319LHsic2NoZW1hIjoiaWdsdTpjb20uY29udml2YVwvY2xpZF9zY2hlbWFcL2pzb25zY2hlbWFcLzEtMC0yIiwiZGF0YSI6eyJpaWQiOiI4MTk0NzE1ODMiLCJjayI6IjI4YmJmMGEzZGVjZjZkMTE2NTAzMmJmZDgzNWQ2ZTE4NDRjNWQ0NGQiLCJjbGlkIjoiMTEwMTU5MTI2NS4yMDUwMTEyMTI2LjI1OTI4NDQwNjYuMzA0ODgzMzYwMyIsImV2ZW50SW5kZXgiOjY2fX0seyJzY2hlbWEiOiJpZ2x1OmNvbS5jb252aXZhLm1vYmlsZVwvYW5kcm9pZF9hcHBfbG9hZHRpbWVcL2pzb25zY2hlbWFcLzEtMC0xIiwiZGF0YSI6eyJvbkNyZWF0ZSI6MTcwMjIwNjc2MTgzNiwiYXBwTG9hZCI6MTcwMjIwNjc2MTAxOSwib25SZXN1bWUiOjE3MDIyMDY3NjIxNDV9fSx7InNjaGVtYSI6ImlnbHU6Y29tLmNvbnZpdmFcL2V2ZW50X2luZm9cL2pzb25zY2hlbWFcLzEtMC0xIiwiZGF0YSI6eyJwcmV2aW91c0V2ZW50VGltZXN0YW1wIjoiMjAyMy0xMi0xMFQxMTozMzoyMi43MTVaIiwiZXZlbnRJbmRleCI6MjV9fSx7InNjaGVtYSI6ImlnbHU6Y29tLmNvbnZpdmFcL2N1c3RvbV90YWdzXC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsiZGF0YSI6e319fSx7InNjaGVtYSI6ImlnbHU6Y29tLnNub3dwbG93YW5hbHl0aWNzLm1vYmlsZVwvc2NyZWVuXC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsiYWN0aXZpdHkiOiJjb20uemVlNS5NYWluQWN0aXZpdHkiLCJuYW1lIjoiY29tLnplZTUuTWFpbkFjdGl2aXR5IiwiaWQiOiI2ZjFkNmE2NC0yYmZjLTQ3NmQtOWRlZC02ZjQyYzkzY2NhNDEiLCJ0eXBlIjoiY29tLnplZTUuTWFpbkFjdGl2aXR5In19LHsic2NoZW1hIjoiaWdsdTpjb20uc25vd3Bsb3dhbmFseXRpY3MubW9iaWxlXC9hcHBsaWNhdGlvbl9saWZlY3ljbGVcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJpbmRleCI6MSwiaXNWaXNpYmxlIjp0cnVlfX1dfQ==\"",\""ue_px\"":\""eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5zbm93cGxvd1wvdW5zdHJ1Y3RfZXZlbnRcL2pzb25zY2hlbWFcLzEtMC0wIiwiZGF0YSI6eyJzY2hlbWEiOiJpZ2x1OmNvbS5zbm93cGxvd2FuYWx5dGljcy5tb2JpbGVcL3NjcmVlbl92aWV3XC9qc29uc2NoZW1hXC8xLTAtMCIsImRhdGEiOnsibmFtZSI6ImNvbS56ZWU1Lk1haW5BY3Rpdml0eSIsInByZXZpb3VzSWQiOiJkYjY0NzBkZi0xNDdiLTRjMjYtYjQyMi0wNDYxODdmNDdkNWMiLCJpZCI6IjZmMWQ2YTY0LTJiZmMtNDc2ZC05ZGVkLTZmNDJjOTNjY2E0MSIsInR5cGUiOiJjb20uemVlNS5NYWluQWN0aXZpdHkiLCJwcmV2aW91c05hbWUiOiJjb20uemVlNS5NYWluQWN0aXZpdHkiLCJwcmV2aW91c1R5cGUiOiJjb20uemVlNS5NYWluQWN0aXZpdHkifX19\"",\""dtm\"":\""1702208002729\"",\""lang\"":\""English\"",\""aid\"":\""Android-Mobile-Zee5\""}]}"", Ip address: ""152.57.209.246"" - src/handlers/event.rs:111""#;

    // Extract the part of the log line containing the serialized JSON for the "body" field
    let start_index = log_line.find("Receive data: b\"").unwrap() + "Receive data: b\"".len();
    let end_index = log_line.find("\", Ip address:").unwrap();
    let body_json = &log_line[start_index..end_index];

    // Deserialize the JSON string into the "Body" structure
    // let body: Body = serde_json::from_str(body_json).unwrap();

    // Now you can access the fields of the deserialized "body" structure
    println!("{:?}", body_json);

    // let decoded_json: Result<String, _> = serde_json::from_str(&body_json);

    // // Check if decoding was successful
    // match decoded_json {
    //     Ok(inner_json) => {
    //         // Parse the inner JSON string
    //         let parsed_json: Result<Value, _> = serde_json::from_str(&inner_json);

    //         // Check if parsing was successful
    //         match parsed_json {
    //             Ok(json_object) => {
    //                 // Successfully parsed JSON, you can now work with the `Value` object
    //                 println!("{:?}", json_object);
    //             }
    //             Err(err) => {
    //                 // Handle parsing error
    //                 println!("Error parsing inner JSON: {:?}", err);
    //             }
    //         }
    //     }
    //     Err(err) => {
    //         // Handle decoding error
    //         println!("Error decoding JSON: {:?}", err);
    //     }
    // }
}
