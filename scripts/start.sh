#!/bin/bash

set -e

BUNDLE_DIR="bundle"
DATABASE_PATH="$BUNDLE_DIR/dependeye.db"

if [ ! -f "$BUNDLE_DIR/dependeye" ]; then
    echo "Application binary not found. Please run ./scripts/build.sh first."
    exit 1
fi

if [ ! -f "$DATABASE_PATH" ]; then
    echo "Database not found. Please run ./scripts/build.sh first."
    exit 1
fi

echo "Starting Dependency Watcher Application..."
DATABASE_URL="sqlite://$DATABASE_PATH" "$BUNDLE_DIR/dependeye"