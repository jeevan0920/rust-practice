struct SelfReferential {
    data: String,
    self_reference: Option<*const String>,
}

impl SelfReferential {
    fn new(data: String) -> Self {
        SelfReferential {
            data,
            self_reference: None,
        }
    }

    fn setup(&mut self) {
        let self_ref: *const String = &self.data; // Get a reference to `data`.
        self.self_reference = Some(self_ref);
    }

    // Unsafe function to dereference the stored pointer.
    // This is unsafe because it can potentially lead to undefined behavior.
    unsafe fn print_self_reference(&self) {
        if let Some(self_ref) = self.self_reference {
            println!("Self reference: {}", &*self_ref); // Dereference the raw pointer.
        }
    }
}

fn take_example(mut example: SelfReferential) {
    // Modify the `data` field after the struct has been moved, simulating use after move.
    example.data = "Modified after move".to_string();
    // Attempt to use the self-reference after the struct has been moved and modified.
    unsafe {
        example.print_self_reference();
    }
}

fn main() {
    let mut example = SelfReferential::new("Hello, world!".to_string());
    example.setup();

    // Move `example`. This would typically "break" `self_reference`, but we won't see it until we try to use it.
    take_example(example);
}
