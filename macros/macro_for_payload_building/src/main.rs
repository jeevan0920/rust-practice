#[macro_use]
extern crate serde_json;

macro_rules! create_json_with_body {
    ($body:expr) => {
        json!({
            "ip_address": "84.28.225.36",
            "timestamp": 1,
            "encoding": "UTF-8",
            "collector": "snowplow-enrich-kafka-3.7.0-common-3.7.0",
            "user_agent": "Mozilla/5.0 (Linux; Android 12; SM-S906N Build/QP1A.190711.020; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/80.0.3987.119 Mobile Safari/537.36",
            "referer_uri": "https://www.google.com",
            "path": "/bb3ae1abc7dd4cb1a558fa78efa2c06615d45818/ctp",
            "querystring": "",
            "body": $body,
            "headers": [
                "Connection-Type: application/json",
                "User-Agent: Mozilla/5.0 (Linux; Android 12; SM-S906N Build/QP1A.190711.020; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/80.0.3987.119 Mobile Safari/537.36",
                "Host: appgw.qe2.conviva.com",
                "Referer: https://www.google.com",
                "Raw-Request-URI: http://appgw.qe2.conviva.com/bb3ae1abc7dd4cb1a558fa78efa2c06615d45818/ctp"
            ],
            "content_type": "application/json",
            "hostname": "appgw.qe2.conviva.com",
            "network_user_id": "swyadav@conviva.com",
            "schema": "iglu:com.snowplowanalytics.snowplow/CollectorPayload/thrift/1-0-0"
        })
    };
}

fn main() {
    let body_content = json!({
        "schema": "iglu:com.snowplowanalytics.snowplow/payload_data/jsonschema/1-0-4",
        "data": [
            {
                "eid": "6b51c182-76de-11ee-ba23-9ea41ba3a606",
                "e": "ue",
                "p": "mob",
                "tv": "ios-0.2.3",
                "tna": "CAT",
                "lang": "zh-CN",
                "tz": "Asia/Shanghai",
                "aid": "Test IOS",
                "res": "1170x2532",
                "vp": "1170x2532",
                "stm": "1698640815040",
                "dtm": "1698640814040",
                "uid": "test2023103001",
                "co": "{\"data\":[{\"data\":{\"eventIndex\":null,\"firstEventId\":\"52b3a792-50a9-49cb-a58e-7f4607cfc19d\",\"firstEventTimestamp\":\"2023-03-10T15:06:18.428Z\",\"previousSessionId\":null,\"sessionId\":\"588410f6-c56b-4172-90e1-15c900e249ef\",\"sessionIndex\":1,\"storageMechanism\":\"LOCAL_STORAGE\"},\"schema\":\"iglu:com.snowplowanalytics.snowplow/client_session/jsonschema/1-0-2\"}],\"schema\":\"iglu:com.snowplowanalytics.snowplow/contexts/jsonschema/1-0-1\"}",
                "ue_pr": "{\"schema\": \"fake_schema\", \"data\": []}"
            }
        ]
    });

    let json_with_body = create_json_with_body!(body_content);

    // Now you can use `json_with_body` for further processing.
    println!("{}", serde_json::to_string_pretty(&json_with_body).unwrap());
}
