use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    gender: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    city: String,
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    person: Vec<Person>,
    location: Location,
}

fn main() {
    // Create sample data
    let sample_data = Data {
        person: vec![
            Person {
                name: "John Doe".to_string(),
                age: 30,
                gender: "Male".to_string(),
            },
            Person {
                name: "Jane Smith".to_string(),
                age: 25,
                gender: "Female".to_string(),
            },
        ],
        location: Location {
            city: "New York".to_string(),
            country: "USA".to_string(),
        },
    };

    // Serialize to XML
    let xml_string = serde_xml_rs::to_string(&sample_data).expect("Failed to serialize to XML");
    println!("Serialized XML:\n{}", xml_string);

    // Deserialize from XML
    let deserialized_data: Data = from_str(&xml_string).expect("Failed to deserialize from XML");
    println!("Deserialized Data: {:#?}", deserialized_data);
}
