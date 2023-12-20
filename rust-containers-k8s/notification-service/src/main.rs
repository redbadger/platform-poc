use std::{thread, time::Duration};

fn main() {
    println!("Notification service started");
    thread::sleep(Duration::from_secs(60 * 60 * 24));
}
