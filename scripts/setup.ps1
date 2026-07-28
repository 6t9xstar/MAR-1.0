# MAR 1.0 first-time setup script for Windows
param()

Write-Host "=== MAR 1.0 Setup ==="

# Copy environment file
if (-not (Test-Path ".env")) {
    Copy-Item ".env.example" ".env"
    Write-Host "Created .env from .env.example"
}

# Install frontend dependencies
Write-Host "Installing frontend dependencies..."
npm install

# Install Python training dependencies
Write-Host "Installing Python training dependencies..."
pip install -r training/requirements.txt

Write-Host ""
Write-Host "=== Setup complete ==="
Write-Host ""
Write-Host "Next steps:"
Write-Host "  1. Start infrastructure: docker compose up -d postgres dragonfly qdrant meilisearch"
Write-Host "  2. Start API server:    cargo run -p api-server"
Write-Host "  3. Start frontend:      npm run dev"
Write-Host "  4. Open browser:        http://localhost:1420"
