#!/bin/bash
# MAR 1.0 development launcher for Linux/macOS
set -euo pipefail

show_help() {
    echo "Usage: ./scripts/dev.sh [OPTION]"
    echo "Options:"
    echo "  --infra       Start Docker infrastructure (postgres, dragonfly, qdrant, meilisearch)"
    echo "  --api         Start the API server"
    echo "  --frontend    Start the frontend dev server"
    echo "  --all         Start everything"
    echo "  --help        Show this help"
}

start_infra() {
    echo "Starting infrastructure..."
    docker compose up -d postgres dragonfly qdrant meilisearch
    echo "Infrastructure ready."
}

start_api() {
    echo "Starting API server..."
    cargo run -p api-server
}

start_frontend() {
    echo "Starting frontend..."
    npm run dev
}

case "${1:-}" in
    --infra)    start_infra ;;
    --api)      start_api ;;
    --frontend) start_frontend ;;
    --all)
        start_infra
        start_api &
        start_frontend
        wait
        ;;
    --help|-h)  show_help ;;
    *)
        echo "Unknown option: $1"
        show_help
        exit 1
        ;;
esac
