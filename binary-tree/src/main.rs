use std::sync::Arc;
use colored_json::ToColoredJson;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use serde_json::json;

#[derive(Debug)]
pub struct Node {
    val: i32,
    left: Option<Arc<Node>>,
    right: Option<Arc<Node>>,
}

// Define a newtype wrapper around Option<Arc<T>>
struct OptionArc<T>(Option<Arc<T>>);

// Implement Serialize for OptionArc<T>
impl<T: Serialize> Serialize for OptionArc<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Some(inner) => inner.serialize(serializer),
            None => serializer.serialize_none(),
        }
    }
}

// Your existing Node struct definition

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 3)?;
        state.serialize_field("val", &self.val)?;
        state.serialize_field("left", &OptionArc(self.left.clone()))?;
        state.serialize_field("right", &OptionArc(self.right.clone()))?;
        state.end()
    }
}






pub fn main() {
    let mut node = Node {
        val: 1,
        left: None,
        right: None,
    };

    let mut left_node = Node {
        val: 2,
        left: None,
        right: None,
    };    

    let mut right_node = Node {
        val: 3,
        left: None,
        right: None,
    };

    // node.left = Arc::new(Some(left_node));
    // node.right = Arc::new(Some(right_node));
    // println!("{:?}", node);
    
    let left_left_node = Node {
        val: 4,
        left: None,
        right: None,
    };

    let left_right_node = Node {
        val: 5,
        left: None,
        right: None,
    };

    left_node.left = Some(Arc::new(left_left_node));
    left_node.right = Some(Arc::new(left_right_node));
    node.left = Some(Arc::new(left_node));
    node.right = Some(Arc::new(right_node));

    let json_data = serde_json::to_string(&node).unwrap();
    println!("{:#?}", json_data);
    // println!("{:#?}", node);
}

