use serde_json::Value;

fn main() {
    // The provided string
    let log_str = r#"""
    """#;

    // Extracting the inner JSON from the "log" field
    let log_json_str = log_str
        .trim_start_matches(r#""{""log"":""#)
        .trim_end_matches(r#""}"#);

    // Unescaping the inner JSON
    let unescaped_json_str = log_json_str.replace(r#"\""#, r#""#);

    // Parsing the JSON
    let log_json: Result<Value, _> = serde_json::from_str(&unescaped_json_str);

    // Handling the result
    match log_json {
        Ok(json) => {
            // Successfully parsed JSON
            println!("Parsed JSON:\n{}", serde_json::to_string_pretty(&json).unwrap());
        }
        Err(e) => {
            // Failed to parse JSON
            eprintln!("Error parsing JSON: {}", e);
        }
    }
}
