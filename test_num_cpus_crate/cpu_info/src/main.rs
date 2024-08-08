use num_cpus;

fn main() {
    println!("Number of CPUs: {}", num_cpus::get());
    println!("Number of physical CPUs: {}", num_cpus::get_physical());

    // Add a long sleep to prevent the application from exiting immediately
    std::thread::sleep(std::time::Duration::from_secs(3600));
}
