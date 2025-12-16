use chrono::Local;
use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use uuid::Uuid;

fn main() {
    println!("Rust Github Logger started");

    loop {
        let timestamp = Local::now().to_rfc3339();
        let uuid = Uuid::new_v4();
        let author: &str = "Hello from KBPsystem777";

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("rust_logs.md")
            .expect("Failed to open rust_logs.md file");

        writeln!(file, "{} | {} | {}", timestamp, uuid, author)
            .expect("Failed to write to rust_logs.md file");

        println!("Log entry added: {} | {} | {}", timestamp, uuid, author);

        thread::sleep(Duration::from_secs(120));
    }
}
