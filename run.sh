#!/bin/bash
# Launch Logistics Workflow
# Default: web-only (server + API). Use --gui for Qt6 desktop.
set -e
cd "$(dirname "$0")"

if [ "$1" = "--gui" ]; then
    echo "Launching Qt6 desktop mode..."
    export QT_QPA_PLATFORM="${QT_QPA_PLATFORM:-xcb}"
    export RUST_LOG="${RUST_LOG:-info}"
    exec cargo run --features gui
else
    echo "Launching web server mode (API on :19876)..."
    echo "Then run: cd web && npm run dev"
    export RUST_LOG="${RUST_LOG:-info}"
    exec cargo run
fi
