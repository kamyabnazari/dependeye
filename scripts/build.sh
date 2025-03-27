#!/bin/bash

set -e

echo "Building dependency watcher application..."

# Build the release binary
cargo build --release

# Copy binary to a dedicated bundle directory
BUNDLE_DIR="bundle"
mkdir -p $BUNDLE_DIR

# Copy binary
cp target/release/app "$BUNDLE_DIR/dependeye"

# Copy configuration file
cp config.toml "$BUNDLE_DIR/"

# Prepare SQLite database
echo "Setting up SQLite database..."
DATABASE_PATH="$BUNDLE_DIR/dependeye.db"
if [ -f "$DATABASE_PATH" ]; then
    echo "Existing database found. Removing old one."
    rm "$DATABASE_PATH"
fi

# Run migrations
SQLX_OFFLINE=true DATABASE_URL="sqlite://$DATABASE_PATH" sqlx migrate run --source backend/migrations

echo "Build complete. Bundle is in the 'bundle/' directory."