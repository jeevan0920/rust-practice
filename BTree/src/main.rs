// Define the BTree node
struct BTreeNode {
    keys: Vec<i32>,
    children: Vec<Option<Box<BTreeNode>>>,
}

// BTree structure
struct BTree {
    root: Option<Box<BTreeNode>>,
}

// BTree methods
impl BTree {
    // Create a new BTree
    fn new() -> Self {
        BTree { root: None }
    }

    // Insert a key into the BTree
    fn insert(&mut self, key: i32) {
        let mut node = self.root.as_mut().unwrap();
        let mut insert_node = true;

        for child in &mut node.children {
            if let Some(ref mut child_node) = child {
                if child_node.keys.len() < 2 {
                    child_node.keys.push(key);
                    insert_node = false;
                    break;
                }
            }
        }

        if insert_node {
            let new_node = Box::new(BTreeNode {
                keys: vec![key],
                children: Vec::new(),
            });
            node.children.push(Some(new_node));
        }
    }

    // Search for a key in the BTree
    fn search(&self, key: i32) -> bool {
        let mut node = self.root.as_ref().unwrap();
        for child in &node.children {
            if let Some(ref child_node) = child {
                for key in &child_node.keys {
                    if *key == *key {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn main() {
    // Create a new BTree
    let mut btree = BTree::new();

    // Insert keys into the BTree
    btree.insert(10);
    btree.insert(20);
    btree.insert(5);

    // Search for a key
    let key_exists = btree.search(10);
    println!("Key 10 exists: {}", key_exists);
}
