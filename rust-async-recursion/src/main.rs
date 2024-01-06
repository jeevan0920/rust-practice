// main.rs

use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use async_recursion::async_recursion;
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};

mod trait_serde;

// Define a trait
trait MyTrait {
    fn validate(&self) -> bool;
}

// Define the SimpleRule struct
#[derive(Debug, Serialize, Deserialize)]
struct SimpleRule {
    name: String,
}

// Define the AnyOfRule struct
#[derive(Debug, Serialize, Deserialize)]
struct AnyOfRule {
    rules: Vec<SimpleRule>,
}

// Define the AllOfRule struct
#[derive(Debug, Serialize, Deserialize)]
struct AllOfRule {
    rules: Vec<SimpleRule>,
}

// Implement MyTrait for SimpleRule
impl MyTrait for SimpleRule {
    fn validate(&self) -> bool {
        // Perform validation logic for SimpleRule
        println!("Validating SimpleRule: {}", self.name);
        // Replace this with your actual validation logic
        true
    }
}

// Implement MyTrait for AnyOfRule
impl MyTrait for AnyOfRule {
    fn validate(&self) -> bool {
        // Perform validation logic for AnyOfRule
        println!("Validating AnyOfRule");
        // Replace this with your actual validation logic
        self.rules.iter().any(|rule| rule.validate())
    }
}

// Implement MyTrait for AllOfRule
impl MyTrait for AllOfRule {
    fn validate(&self) -> bool {
        // Perform validation logic for AllOfRule
        println!("Validating AllOfRule");
        // Replace this with your actual validation logic
        self.rules.iter().all(|rule| rule.validate())
    }
}

// Define a recursive validation rule enum
#[derive(Debug, Serialize, Deserialize)]
enum RecursiveRule {
    Simple(SimpleRule),
    AnyOf(AnyOfRule),
    AllOf(AllOfRule),
    Recursive(Box<RecursiveRule>),
}

// Implement MyTrait for RecursiveRule
impl MyTrait for RecursiveRule {
    fn validate(&self) -> bool {
        match self {
            RecursiveRule::Simple(inner) => inner.validate(),
            RecursiveRule::AnyOf(inner) => inner.validate(),
            RecursiveRule::AllOf(inner) => inner.validate(),
            RecursiveRule::Recursive(inner) => inner.validate(),
        }
    }
}

// Modify the async recursive function to take a JSON input
#[async_recursion]
async fn recursive_function(
    rule: RecursiveRule,
    depth: usize,
) -> Arc<Mutex<dyn MyTrait + Send>> {
    if depth == 0 {
        // Base case: return a validation rule based on the provided JSON
        Arc::new(Mutex::new(rule)) as Arc<Mutex<dyn MyTrait + Send>>
    } else {
        // Recursive case: call the function recursively and return the validation rule
        sleep(Duration::from_secs(1)).await; // Simulating asynchronous work

        let inner_result = recursive_function(rule, depth - 1).await;
        Arc::new(Mutex::new(RecursiveRule::Recursive(Box::new(inner_result))))
            as Arc<Mutex<dyn MyTrait + Send>>
    }
}

#[tokio::main]
async fn main() {
    // Sample JSON input for validation rules
    let json_input = r#"
        {
            "AnyOf": [
                {
                    "Simple": {"name": "Rule1"}
                },
                {
                    "AllOf": [
                        {"Simple": {"name": "Rule2"}},
                        {"Simple": {"name": "Rule3"}}
                    ]
                }
            ]
        }
    "#;

    // Parse JSON input into RecursiveRule
    let parsed_rule: RecursiveRule = serde_json::from_str(json_input).unwrap();

    // Call the async recursive function
    let result = recursive_function(parsed_rule, 3).await;

    // Validate the result
    let validation_result = result.lock().unwrap().validate();
    println!("Validation Result: {}", validation_result);
}
