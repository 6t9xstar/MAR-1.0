# MAR 1.0 Development Script (Windows)
param(
    [switch]$Infra,
    [switch]$Api,
    [switch]$Frontend,
    [switch]$Desktop,
    [switch]$All
)

$RootDir = Split-Path $PSScriptRoot -Parent

if ($Infra -or $All) {
    Write-Host "Starting infrastructure..." -ForegroundColor Green
    docker compose -f "$RootDir/docker-compose.yml" up -d postgres dragonfly qdrant meilisearch
    Write-Host "Infrastructure started" -ForegroundColor Green
}

if ($Api -or $All) {
    Write-Host "Starting API server..." -ForegroundColor Green
    $env:RUST_LOG = "info"
    $job = Start-Job -ScriptBlock {
        Set-Location $using:RootDir
        cargo run -p api-server
    }
    Write-Host "API server starting in background (PID: $($job.Id))" -ForegroundColor Green
}

if ($Frontend -or $All) {
    Write-Host "Starting frontend dev server..." -ForegroundColor Green
    Set-Location $RootDir
    npm run dev
}

if ($Desktop -or $All) {
    Write-Host "Starting desktop app..." -ForegroundColor Green
    Set-Location $RootDir
    npm run tauri:dev
}

if (-not ($Infra -or $Api -or $Frontend -or $Desktop -or $All)) {
    Write-Host @"
MAR 1.0 Development Script
Usage: .\scripts\dev.ps1 [options]

Options:
  -Infra     Start infrastructure (PostgreSQL, DragonflyDB, Qdrant, Meilisearch)
  -Api       Start API server
  -Frontend  Start frontend dev server
  -Desktop   Start Tauri desktop app
  -All       Start everything

Examples:
  .\scripts\dev.ps1 -Infra
  .\scripts\dev.ps1 -Api -Frontend
  .\scripts\dev.ps1 -All
"@
}
