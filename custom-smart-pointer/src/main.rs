use std::ops::{Deref, DerefMut};

struct MyBox<T> {
    val: T,
}

impl<T> MyBox<T> {
    fn new(val: T) -> MyBox<T> {
        MyBox { val }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox is droppped");
    }
}

fn main() {
    println!("outside the scope");
    {
        let mut bx = MyBox::new(2);
        println!("deref the box: {}", *bx);

        *bx = 3;
        println!("deref the box after modifying it: {}", *bx);
    }
    println!("outside the scrope");
}
