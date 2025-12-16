# rusty-github-logger

An educational implementation of an automated GitHub commit logger written in Rust — intended for learning and demonstration purposes only.

---

## Purpose ✅
This repository is a simple learning project that demonstrates basic Rust concepts: writing to files, generating UUIDs, working with time using `chrono`, spawning child processes with `std::process::Command`, and running periodic tasks in a loop. It is intended for education and experimentation only, not production use.

## What the program does 🧭
- Appends a log entry to `rust_logs.md` in the format: `YYYY-MM-DDTHH:MM:SS±HH:MM | <UUID> | <author>`
- Generates a new UUID for each entry and prints it to stdout
- Runs an automated sequence for every entry (by default every 60 seconds):
  - `git add rust_logs.md`
  - `git commit -m "⚡Auto-update <UUID>"`
  - `git push origin main`

> ⚠️ Warning: the program will execute `git push`. Make sure you run it in a repository/branch you control and that your credentials are configured safely.

## Expectations (what you should see) 🔍
- A file called `rust_logs.md` will be created (if absent) and appended to on each run.
- Commits will be created and pushed to the `origin` remote on the `main` branch using commit messages that include the generated UUID.
- The process runs in an infinite loop and sleeps for 60 seconds between iterations (adjustable in `src/main.rs`).

## Prerequisites 🔧
- Git installed and configured with a remote `origin` where you have push access.
- Rust toolchain installed (via `rustup` and `cargo`).
- Optional: SSH keys or credential helper configured for non-interactive pushes.

## Install Rust
Follow the official installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# then reload your shell or run:
source "$HOME/.cargo/env"
```

Verify installation:

```bash
rustc --version
cargo --version
```

## Get the code & build
```bash
git clone <your-repo-url>
cd rust-github-commit-logger
cargo build --release
```

## Run locally ▶️
Start the app in the foreground:

```bash
cargo run
# or run the release binary
cargo run --release
```

- The app will append to `rust_logs.md` and attempt to commit & push each new entry.
- Stop with `Ctrl+C`.

## Configuration & customization 🔧
- Change the sleep duration by editing `Duration::from_secs(60)` in `src/main.rs`.
- Change the branch/remote used for pushing in `run_git_commands()` if you need a different target.
- For safer testing, comment out or remove the `git push` step until you're comfortable with the commits.

## Safety & testing recommendations 🧪
- Test with a disposable repository or on a separate branch.
- Consider replacing the push with a dry-run or printing commands during development.

## Contributing
This is a learning project — contributions that improve clarity, safety, or educational value are welcome. Open an issue or a PR with changes.

## License
See the `LICENSE` file for license details.
