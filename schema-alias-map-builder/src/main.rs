use base64::engine::general_purpose;
use base64::Engine;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_bijective_map(map: &HashMap<String, String>) -> bool {
    // print the map
    println!("map: {:?}", map);
    let mut value_set = HashSet::new();
    println!("value_set: {:?}", map.values());
    for value in map.values() {
        if !value_set.insert(value) {
            // If the value was already in the set, it's not bijective
            // TODO: print the error log here
            return false;
        }
    }
    true
}

fn build_alias_map() -> Result<HashMap<String, String>, Error> {
    // Open the file
    let file = match File::open(std::env::var("ALIAS_MAP_PATH").unwrap()) {
        Ok(file) => file,
        Err(e) => {
            panic!(
                "Failed to open alias map file {} : {}",
                std::env::var("ALIAS_MAP_PATH").unwrap(),
                e
            );
        }
    };

    // Create a buffered reader to read lines efficiently
    let reader = BufReader::new(file);

    // Create a HashMap to store the data
    let mut hashmap: HashMap<String, String> = HashMap::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                panic!("Failed to read line: {}", e);
            }
        };

        // Split the line into key and value parts
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            // Insert key-value pair into the HashMap
            hashmap.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
        } else {
            println!("Invalid line: {}", line);
        }
    }

    if !is_bijective_map(&hashmap) {
        panic!("Mapping is not bijective");
    }

    // Print the HashMap
    for (key, value) in &hashmap {
        println!("{} => {}", key, value);
    }

    Ok(hashmap)
}

fn alias_decode(mut json_body: &mut Value, alias_map: &HashMap<String, String>) {
    // correcte the aliased data key
    if let Some(aliased_data) = alias_map.get("data") {
        if json_body[aliased_data].is_object() {
            // correct the key name
            json_body["data"] = json_body[aliased_data].clone();
            // remove the old key
            if let Some(json_body_obj) = json_body.as_object_mut() {
                json_body_obj.remove(aliased_data);
            }
        }
    }

    // correct the aliased schema key
    if let Some(aliased_schema) = alias_map.get("schema") {
        if !json_body[aliased_schema].is_null() {
            let aliased_schema_value = json_body[aliased_schema].clone();
            // correct the key name
            json_body["schema"] = json_body[aliased_schema].clone();
            // remove the old key
            if let Some(json_body_obj) = json_body.as_object_mut() {
                json_body_obj.remove(aliased_schema);
            }

            // correct the aliased schema value
            if let Some(aliased_schema_value) = alias_map.get(&aliased_schema_value.to_string()) {
                if let Ok(corrected_schema_value) = serde_json::from_str(aliased_schema_value) {
                    json_body["schema"] = corrected_schema_value;
                }
            }
        }
    }

    // recursively correct the aliased data value
    if let Some(data_array) = json_body["data"].as_array_mut() {
        for obj in data_array {
            alias_decode(obj, alias_map);
        }
    }
}

fn decode_aliased_payload(alias_map: &HashMap<String, String>, payload: &str) -> Value {
    // Decode the payload from base64
    match general_purpose::STANDARD.decode(payload) {
        Ok(_payload) => {
            // get the clarity from sensor about this
            todo!()
        }
        Err(_e) => {
            // this is expected
        }
    };

    let mut json_body: Value = serde_json::from_str(payload).unwrap();

    // check that the json is a valid json
    if !json_body.is_object() {
        panic!("Invalid JSON: {}", json_body);
    }

    alias_decode(&mut json_body, alias_map);

    json_body
}

fn main() {
    let _alias_map = build_alias_map().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bijective_map() {
        let mut map = HashMap::new();
        map.insert("a".to_string(), "1".to_string());
        map.insert("b".to_string(), "2".to_string());
        map.insert("c".to_string(), "3".to_string());
        assert_eq!(is_bijective_map(&map), true);

        map.insert("d".to_string(), "1".to_string());
        assert_eq!(is_bijective_map(&map), false);
    }

    #[ignore]
    #[test]
    fn test_build_alias_map() {
        todo!("fill it from the final mapping file and test it.")
    }

    #[test]
    fn test_decode_alias_without_base64() {
        // build a json from string
        let json_string = "{\n\t\"sch\": \"iglu:com.snowplowanalytics.snowplow\\/payload_data\\/jsonsch\\/1-0-4\",\n\t\"data\": [{\n\t\t\"eid\": \"0377d9d2-c66a-48dc-ada2-9522a0c0044f\",\n\t\t\"res\": \"1080x2205\",\n\t\t\"tv\": \"andr-0.8.3-rc3\",\n\t\t\"e\": \"ue\",\n\t\t\"tna\": \"CAT\",\n\t\t\"tz\": \"Asia\\/Kolkata\",\n\t\t\"co\": \"{\\\"sch\\\":\\\"iglu:com.snowplowanalytics.snowplow\\\\\\/contexts\\\\\\/jsonsch\\\\\\/1-0-1\\\",\\\"data\\\":[{\\\"sch\\\":\\\"iglu:com.snowplowanalytics.snowplow\\\\\\/client_session\\\\\\/jsonsch\\\\\\/1-0-2\\\",\\\"data\\\":{\\\"sessionIndex\\\":1,\\\"storageMechanism\\\":\\\"LOCAL_STORAGE\\\",\\\"firstEventTimestamp\\\":\\\"2024-02-20T09:12:22.810Z\\\",\\\"firstEventId\\\":\\\"9b152154-af7a-4952-aa55-d28ca49f42d4\\\",\\\"sessionId\\\":\\\"32b00273-98e0-479a-ba7b-bf828940834d\\\",\\\"eventIndex\\\":3,\\\"previousSessionId\\\":null,\\\"userId\\\":\\\"77e150e3-c21d-487b-92db-dd1ac30398bc\\\"}},{\\\"sch\\\":\\\"iglu:com.snowplowanalytics.mobile\\\\\\/application\\\\\\/jsonsch\\\\\\/1-0-0\\\",\\\"data\\\":{\\\"build\\\":\\\"1\\\",\\\"version\\\":\\\"0.8.3-rc3\\\"}},{\\\"sch\\\":\\\"iglu:com.snowplowanalytics.snowplow\\\\\\/mobile_context\\\\\\/jsonsch\\\\\\/1-0-2\\\",\\\"data\\\":{\\\"osVersion\\\":\\\"14\\\",\\\"batteryState\\\":\\\"unplugged\\\",\\\"osType\\\":\\\"android\\\",\\\"androidIdfa\\\":\\\"6527834b-05e0-4318-881d-ba70e77dc6b5\\\",\\\"deviceModel\\\":\\\"Pixel 6a\\\",\\\"deviceManufacturer\\\":\\\"Google\\\",\\\"networkType\\\":\\\"wifi\\\",\\\"batteryLevel\\\":94}},{\\\"sch\\\":\\\"iglu:com.conviva\\\\\\/clid_sch\\\\\\/jsonsch\\\\\\/1-0-2\\\",\\\"data\\\":{\\\"iid\\\":\\\"1141744339\\\",\\\"ck\\\":\\\"4d2f03dddf417990f520f09d79b11ab014c39dab\\\",\\\"clid\\\":\\\"348334540.964030925.100274147.1008687406\\\",\\\"eventIndex\\\":3}},{\\\"sch\\\":\\\"iglu:com.conviva.mobile\\\\\\/android_app_loadtime\\\\\\/jsonsch\\\\\\/1-0-1\\\",\\\"data\\\":{\\\"onCreate\\\":1708420342797,\\\"appLoad\\\":1708420342653,\\\"onResume\\\":1708420342941}},{\\\"sch\\\":\\\"iglu:com.conviva\\\\\\/event_info\\\\\\/jsonsch\\\\\\/1-0-1\\\",\\\"data\\\":{\\\"previousEventTimestamp\\\":\\\"2024-02-20T09:12:25.231Z\\\",\\\"eventIndex\\\":3}},{\\\"sch\\\":\\\"iglu:com.conviva\\\\\\/custom_tags\\\\\\/jsonsch\\\\\\/1-0-0\\\",\\\"data\\\":{\\\"data\\\":{\\\"hi\\\":\\\"kirankumar\\\",\\\"int1\\\":\\\"0\\\",\\\"test\\\":\\\"pass\\\",\\\"double\\\":\\\"1234.1\\\",\\\"double1\\\":\\\"0.0\\\",\\\"boolean2\\\":\\\"false\\\",\\\"boolean1\\\":\\\"true\\\",\\\"int\\\":\\\"1234\\\"}}},{\\\"sch\\\":\\\"iglu:com.conviva\\\\\\/applied_remote_config\\\\\\/jsonsch\\\\\\/1-0-0\\\",\\\"data\\\":{\\\"cacheRefreshIntervalInSec\\\":3600,\\\"configAppliedTime\\\":1708420345162,\\\"source\\\":\\\"remote\\\",\\\"version\\\":27585}},{\\\"sch\\\":\\\"iglu:com.snowplowanalytics.mobile\\\\\\/screen\\\\\\/jsonsch\\\\\\/1-0-0\\\",\\\"data\\\":{\\\"activity\\\":\\\"com.conviva.hp.demoapp.MainActivity\\\",\\\"name\\\":\\\"com.conviva.hp.demoapp.MainActivity\\\",\\\"id\\\":\\\"94f78a6d-fdfd-46ab-95ab-f48c73d049d9\\\",\\\"type\\\":\\\"com.conviva.hp.demoapp.MainActivity\\\"}},{\\\"sch\\\":\\\"iglu:com.conviva.mobile\\\\\\/android_screen_loadtime\\\\\\/jsonsch\\\\\\/1-0-0\\\",\\\"data\\\":{\\\"onCreate\\\":1708420342797,\\\"onResume\\\":1708420342941}},{\\\"sch\\\":\\\"iglu:com.conviva.mobile\\\\\\/bundle_info\\\\\\/jsonsch\\\\\\/1-0-0\\\",\\\"data\\\":{\\\"info\\\":\\\"{\\\\\\\"action\\\\\\\":\\\\\\\"android.intent.action.MAIN\\\\\\\",\\\\\\\"categories\\\\\\\":[\\\\\\\"android.intent.category.LAUNCHER\\\\\\\"]}\\\"}},{\\\"sch\\\":\\\"iglu:com.snowplowanalytics.mobile\\\\\\/application_lifecycle\\\\\\/jsonsch\\\\\\/1-0-0\\\",\\\"data\\\":{\\\"isVisible\\\":true}}]}\",\n\t\t\"stm\": \"1708420345400\",\n\t\t\"p\": \"mob\",\n\t\t\"ue_pr\": \"{\\\"sch\\\":\\\"iglu:com.snowplowanalytics.snowplow\\\\\\/unstruct_event\\\\\\/jsonsch\\\\\\/1-0-0\\\",\\\"data\\\":{\\\"sch\\\":\\\"iglu:com.conviva\\\\\\/raw_event\\\\\\/jsonsch\\\\\\/1-0-1\\\",\\\"data\\\":{\\\"data\\\":\\\"{\\\\\\\"id\\\\\\\":1234,\\\\\\\"islive\\\\\\\":true,\\\\\\\"asset\\\\\\\":\\\\\\\"Movie\\\\\\\"}\\\",\\\"name\\\":\\\"asset_metadata\\\"}}}\",\n\t\t\"dtm\": \"1708420345272\",\n\t\t\"lang\": \"English\",\n\t\t\"aid\": \"Test_BuildDemo\"\n\t}]\n}";
        let alias_map = build_alias_map().unwrap();
        let decoded_json = decode_aliased_payload(&alias_map, json_string);
        println!("decoded_json: {:#?}", decoded_json);
    }

    #[ignore]
    #[test]
    fn test_base64_decode_with_plain_string() {
        todo!("work with sensor team to get the clarity on this.")
    }
}
