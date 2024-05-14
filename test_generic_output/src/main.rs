fn main() {
    println!("Hello, world!");
    let mut event = CISUeEvent::new("abc".to_string());
    event = replace_event(event);
    println!("{:?}", event);
}

#[derive(Debug)]
pub struct CISUeEvent<T> {
    pub schema: String,
    pub data: T,
}

impl<T> CISUeEvent<T> {
    pub fn new(event: T) -> Self {
        CISUeEvent {
            schema: "".to_string(),
            data: event,
        }
    }
}

// This function creates and returns a new event
fn replace_event<T>(_: CISUeEvent<T>) -> CISUeEvent<String> {
    CISUeEvent::new("abcd".to_string())
}
