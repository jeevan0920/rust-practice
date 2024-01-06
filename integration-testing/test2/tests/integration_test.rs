use test2::test;  // Import the tests module

#[test]
fn it_adds_two() {
    test::helper_function();  // Access the public helper function
    assert_eq!(4, test2::add_two(2));
}
