#!/bin/bash
set -e

echo "Building dependeye application..."

# Build the release binary
cargo build --release

# Create the bundle directory
BUNDLE_DIR="$(pwd)/bundle"
mkdir -p "$BUNDLE_DIR"

# Copy the compiled binary and rename it to "dependeye"
cp target/release/app "$BUNDLE_DIR/dependeye"

# Copy configuration file
cp Config.toml "$BUNDLE_DIR/"

# Set up the SQLite database and apply migrations using SQLx CLI
DATABASE_PATH="$BUNDLE_DIR/dependeye.db"
echo "Setting up SQLite database..."
sqlx database create --database-url "sqlite:///$DATABASE_PATH"
sqlx migrate run --source backend/migrations --database-url "sqlite:///$DATABASE_PATH"

echo "Build complete. Bundle is in the 'bundle/' directory."