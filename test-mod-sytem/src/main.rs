mod grand_parent;

fn main() {
    println!("Hello, world!");

    // Call the function from child1.rs
    grand_parent::parent1::child1::message();

    // call the function from grand_child1.rs
    grand_parent::parent1::child1::grand_child1::message();
}
