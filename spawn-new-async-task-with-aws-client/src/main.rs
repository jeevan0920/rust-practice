use actix_rt::System;
use awc::Client;

fn main() {
    let system = System::new();

    system.block_on(async {
        actix_rt::spawn(async {
            let client = Client::default();

            let response = client
                .get("https://example.com")
                .send()
                .await
                .expect("Failed to send request");

            // Process the response here
            println!("Response status: {}", response.status());
        });

        // Since the task is spawned, it runs independently of the main flow
        // You can perform other operations here or just wait for the system to finish
    });
}
