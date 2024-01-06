// module1.rs

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("This is the macro in 'module1'");
    };
}

pub fn use_macro() {
    my_macro!();
}
