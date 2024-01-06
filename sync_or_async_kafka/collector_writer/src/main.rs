use rdkafka::producer::{BaseProducer, ProducerConfig};

fn main() {
    // Create a producer configuration.
    let config = ProducerConfig::new()
        .set_bootstrap_servers("localhost:9092")
        .build();

    // Create a new producer.
    let producer = BaseProducer::create(&config).unwrap();

    // Create a new record to produce.
    for i in 0..100 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        let record = rdkafka::producer::BaseRecord::to("my-topic");

        // Produce the record.
        producer.send(&record).unwrap();
    }

    // Flush the producer.
    producer.flush().unwrap();
}
