use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct YourStruct {
    name: String,
    age: i32,
}

fn main() {
    let your_struct = YourStruct {
        name: "John".to_string(),
        age: 30,
    };

    // Serialize the struct to a JSON string
    let json_string = serde_json::to_string(&your_struct).unwrap();
    println!("{}", json_string);
}
