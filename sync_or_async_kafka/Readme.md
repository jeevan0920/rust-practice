// This is is an experiement to understand sync/async runtimes in Rust
// In this project I will write some messages to kafka


// I will read these messages from a sync kakfa with more number of workers (than the number of threads available in my CPU) and see and how it is going to behave