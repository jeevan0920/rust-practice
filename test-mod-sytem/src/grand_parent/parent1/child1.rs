pub mod grand_child1;

pub fn message() {
    println!("Hello from grand_parent/parent1/parent1_child.rs");

    // Call the function from parent2_child.rs
    super::super::parent2::child2::secret_message();
}
