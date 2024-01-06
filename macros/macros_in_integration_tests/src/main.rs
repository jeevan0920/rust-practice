extern crate serde;
extern crate serde_json;

use serde_json::{Value, Error};

fn main() -> Result<(), Error> {
    let json_str = r#"
    {
        "name": "John",
        "age": 30,
        "city": "New York"
    }
    "#;

    let mut data: Value = serde_json::from_str(json_str)?;

    // Create a new object to insert
    let new_object = serde_json::json!({
        "key1": "value1",
        "key2": 42,
        "key3": true
    });

    // Insert the new object into the JSON structure
    data["new_object"] = new_object;

    // Serialize the modified data back to JSON
    let updated_json = serde_json::to_string(&data)?;

    println!("{}", updated_json);

    Ok(())
}
