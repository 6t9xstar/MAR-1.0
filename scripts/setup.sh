#!/bin/bash
# MAR 1.0 first-time setup script for Linux/macOS
set -euo pipefail

echo "=== MAR 1.0 Setup ==="

# Copy environment file
if [ ! -f .env ]; then
    cp .env.example .env
    echo "Created .env from .env.example"
fi

# Install frontend dependencies
echo "Installing frontend dependencies..."
npm install

# Install Python training dependencies
echo "Installing Python training dependencies..."
pip install -r training/requirements.txt

echo ""
echo "=== Setup complete ==="
echo ""
echo "Next steps:"
echo "  1. Start infrastructure: docker compose up -d postgres dragonfly qdrant meilisearch"
echo "  2. Start API server:    cargo run -p api-server"
echo "  3. Start frontend:      npm run dev"
echo "  4. Open browser:        http://localhost:1420"
