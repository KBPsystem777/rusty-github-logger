use chrono::Local;
use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use uuid::Uuid;

fn main() {
    println!("🦀🦀🦀 Rust Github Logger started");

    loop {
        let timestamp = Local::now().to_rfc3339();
        let uuid = Uuid::new_v4();
        let author: &str = "Hello from @KBPsystem777";

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("rust_logs.md")
            .expect("Failed to open rust_logs.md file");

        writeln!(file, "{} | {} | {}", timestamp, uuid, author)
            .expect("Failed to write to rust_logs.md file 🫠");

        run_git_commands(&uuid);

        println!("🏗️ Log entry added: {} | {} | {}", timestamp, uuid, author);

        thread::sleep(Duration::from_secs(60));
    }
}

use std::process::Command;

fn run_git_commands(uuid: &uuid::Uuid) {
    // Git add
    let result = Command::new("git")
        .args(&["add", "rust_logs.md"])
        .output()
        .expect("Failed to execute git add command");
    if !result.status.success() {
        eprint!("Git add error: {}", String::from_utf8_lossy(&result.stderr))
    }

    // Git commit
    let commit_message = format!("⚡Auto-update {}", uuid);
    let result = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("Failed to execute git commit command");
    if !result.status.success() {
        eprint!(
            "Git commit error: {}",
            String::from_utf8_lossy(&result.stderr)
        )
    }

    let result = Command::new("git")
        .args(&["push", "origin", "main"])
        .output()
        .expect("Failed to execute git push");
    if !result.status.success() {
        eprint!(
            "Git push error: {}",
            String::from_utf8_lossy(&result.stderr)
        )
    }
}
