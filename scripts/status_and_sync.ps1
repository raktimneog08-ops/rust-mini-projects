# Progress and Sync Helper for 100 Rust Mini Projects
# Checks implementation status, runs workspace quality gates, and pushes to remote.

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "🦀 100 Rust Mini Projects Status Tracker" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$dirs = Get-ChildItem -Directory | Where-Object { $_.Name -match '^\d{2,3}-' } | Sort-Object Name

$completed = @()
$pending = @()

foreach ($dir in $dirs) {
    $hasCargo = Test-Path (Join-Path $dir.FullName "Cargo.toml")
    if ($hasCargo) {
        $completed += $dir.Name
    } else {
        $pending += $dir.Name
    }
}

Write-Host ""
Write-Host "✅ Completed Projects: $($completed.Count) / $($dirs.Count)" -ForegroundColor Green
foreach ($c in $completed) {
    Write-Host "   - $c" -ForegroundColor Green
}

Write-Host ""
Write-Host "⏳ Pending Projects:   $($pending.Count) / $($dirs.Count)" -ForegroundColor Yellow
if ($pending.Count -gt 0) {
    Write-Host "👉 Next in Queue:     $($pending[0])" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Verifying workspace health..." -ForegroundColor Cyan
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Formatting check failed!" -ForegroundColor Red
    exit 1
}

cargo clippy --workspace --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Clippy linter failed!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Workspace is 100% healthy!" -ForegroundColor Green

# Optional Push to Remote
$remote = git remote get-url origin 2>$null
if ($remote) {
    Write-Host ""
    Write-Host "Attempting push to origin main..." -ForegroundColor Cyan
    git push origin main --tags
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to push to remote repository ($remote)!" -ForegroundColor Red
        Write-Host "   Please ensure the repository exists on GitHub and git credentials are authenticated." -ForegroundColor Yellow
        exit $LASTEXITCODE
    } else {
        Write-Host "🚀 Successfully pushed to origin main with tags!" -ForegroundColor Green
    }
}
