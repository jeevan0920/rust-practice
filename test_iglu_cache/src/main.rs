use std::collections::HashMap;
use serde::Deserialize;
use serde_json::{Value, Error};
use jsonschema::{JSONSchema, Draft};
use std::io::{Read, Write};
use std::fs::{File, create_dir_all};
use std::path::Path;
use http::StatusCode;
use log::{info, error};
use reqwest::blocking::{Client, Response};
use reqwest::header::HeaderValue;
use std::fs;
use std::io::prelude::*;

pub struct IgluServer {
    endpoint: String,
    auth_key: String,
}

impl IgluServer {
    pub fn new(
        endpoint: String,
        auth_key: String,
    ) -> Self {
        IgluServer {
            endpoint,
            auth_key,
        }
    }

    fn load_schemas_from_directory(directory_path: &str) -> HashMap<String, JSONSchema> {
        let mut schema_map = HashMap::new();

        // Read the directory and iterate through its entries
        if let Ok(entries) = fs::read_dir(directory_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Get the file path and its filename (schema ID)
                    let file_path = entry.path();
                    if let Some(file_name) = file_path.file_name().and_then(|name| name.to_str()) {
                        dbg!(file_name);
                        // Read the contents of the file line by line
                        let mut file = fs::File::open(file_path.clone()).expect("Failed to open file");
                        let mut file_contents = String::new();
                        file.read_to_string(&mut file_contents).expect("Failed to read file");

                        // Split the contents into lines
                        let lines: Vec<&str> = file_contents.lines().collect();
                        if lines.len() >= 2 {
                            let schema_id = lines[0].to_string();
                            let schema_json = lines[1].to_string();

                            // Deserialize the schema JSON string into a serde_json::Value
                            let schema_value: Value = serde_json::from_str(&schema_json).expect("Failed to deserialize schema");

                            // Create a JSONSchema from the Value and insert it into the HashMap with the schema ID as the key
                            if let Ok(schema) = JSONSchema::compile(&schema_value) {
                                schema_map.insert(schema_id, schema);
                            } else {
                                eprintln!("Failed to compile schema: {}", schema_id);
                            }
                        } else {
                            eprintln!("Invalid file format: {}", file_name);
                        }
                    }
                }
            }
        } else {
            eprintln!("Failed to read directory: {}", directory_path);
        }

        schema_map
    }

    fn validate_request(schema_map: &HashMap<String, JSONSchema>, json_str: &str) -> Result<(), String> {
        // Deserialize the incoming JSON request into a serde_json::Value
        let request_json: Value = serde_json::from_str(json_str).map_err(|e| e.to_string())?;

        // Get the schema identifier from the request JSON
        let schema_key = request_json.get("$schema").and_then(|value| value.as_str());

        // If the schema identifier is not found, return an error
        if schema_key.is_none() {
            return Err("No schema identifier found in JSON request".to_string());
        }

        // Find the corresponding schema in the HashMap using the schema identifier
        let schema = schema_map.get(schema_key.unwrap());

        // If the schema is not found, return an error
        if schema.is_none() {
            return Err("Schema not found for the given identifier".to_string());
        }

        // Validate the JSON request against the schema
        let x = match schema.unwrap().validate(&request_json) {
            Ok(_) => Ok(()),
            Err(errors) => Err(format!("Validation errors:")),
        }; 
        x
    }
}

fn main() {
    println!("Hello, world!");

    // Set the environment variable "CONVIVA_IGLU_ENDPOINT"
    std::env::set_var("CONVIVA_IGLU_ENDPOINT", "https://iglu-iad5.conviva.com/api/schemas/com.conviva");

    // Now the environment variable is set and you can use it in your code as needed
    let conviva_iglu_endpoint = std::env::var("CONVIVA_IGLU_ENDPOINT").unwrap_or_default();

    // Read the environment variable "conviva_iglu_api_key"
    if let Ok(conviva_iglu_api_key) = std::env::var("conviva_iglu_api_key") {
        // Use the conviva_iglu_api_key here in your code as needed
        println!("convivaIgluApiKey: {}", conviva_iglu_api_key);

        // Create the IgluServer with the provided endpoint and API key
        let iglu_server = IgluServer::new(conviva_iglu_endpoint, conviva_iglu_api_key);
        // Rest of your code using the iglu_server...
    } else {
        // The environment variable is not set or an error occurred while reading it
        println!("convivaIgluApiKey is not set.");

        // Handle the case where the API key is not set, e.g., show an error or exit gracefully.
        // For example:
        // return; // or any other graceful exit strategy
    }
    
    IgluServer::load_schemas_from_directory("/Users/mkumar/work/KafkaWriterTLB/local/conviva_iglu");
}
