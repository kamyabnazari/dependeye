# DependEye

A simple desktop app to track the health of your GitHub dependencies.

Built with Rust, egui (for the UI), and SQLite (for local data storage).

## Features

- Fetch GitHub repo data (stars, forks, issues)
- Store and view dependencies locally
- Works offline after initial sync

## Requirements

- Rust: https://rustup.rs
- (Optional) GitHub API token for higher rate limits
- SQLx CLI for migrations:

```bash
cargo install sqlx-cli --no-default-features --features sqlite
```

Clone Project

```bash
git clone https://github.com/kamyabnazari/dependeye.git
```

## Build the app:

```bash
bash scripts/build.sh
```

## Start the app:

```bash
bash scripts/start.sh
```

## Project Layout
- app/ - Desktop app entry
- backend/ - GitHub + database logic
- frontend/ - UI code using egui
- scripts/ - Build/start scripts
- bundle/ - Output folder (binary + database)

## Notes
- Uses SQLite only
- All data is stored locally
- No network needed after first fetch