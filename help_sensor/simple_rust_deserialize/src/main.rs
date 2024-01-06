use regex::Regex;

fn main() {
    // Your log string
    let log_line = "2023-12-14 14:53:47.557 - [1;34mINFO[0m [gateway::handlers::event] - Universal Identity Headers: HeaderMap { inner: {...}, Receive body size: 3687, Receive data: b\"{\\\"schema\\\":\\\"iglu:com.snowplowanalytics.snowplow\\/payload_data\\/jsonschema\\/1-0-4\\\",\\\"data\\\":[{\\\"eid\\\":\\\"032e6af7-d1d8-4f1e-bf75-820b86ba11cb\\\", ... }\", Ip address: \"152.57.209.246\" - src/handlers/event.rs:111";

    // Define the regex pattern
    let pattern = r"Receive data: b\"(\{.*?\})\"";

    // Create a regex object
    let re = Regex::new(pattern).expect("Failed to create regex");

    // Find the match using the regex
    if let Some(captures) = re.captures(log_line) {
        // Extract the content within the curly braces
        if let Some(body_content) = captures.get(1) {
            // Print the extracted "body"
            println!("{}", body_content.as_str());
        }
    } else {
        println!("No match found.");
    }
}
