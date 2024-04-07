pub fn message() {
    println!("Hello from grand_parent/parent1/child1/grand_child1.rs");

    // Call the function from parent2_child.rs
    crate::grand_parent::parent2::child2::grand_child2::secret_message();
}
