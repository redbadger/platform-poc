use std::{thread, time::Duration};

fn main() {
    println!("Product service started");
    thread::sleep(Duration::from_secs(60 * 60 * 24));
}
