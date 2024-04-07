use jsondata::Json;

fn main() {
    // Parse JSON texts into jsondata::Json instances
    let json_text1 = r#"{"key2": 2, "key1": 1}"#;
    let json_text2 = r#"{"key1": 1, "key2": 2}"#;

    let json1 = json_text1.parse::<Json>().unwrap();
    let json2 = json_text2.parse::<Json>().unwrap();

    // Compare the entire content of the JSON objects
    if json1 == json2 {
        println!("The JSONs are equal.");
    } else {
        println!("The JSONs are not equal.");
    }

    assert_eq!(json1, json2);
}
