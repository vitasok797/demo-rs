cargo --config 'target.x86_64-pc-windows-msvc.rustflags=[\"-C\", \"target-feature=+crt-static\"]' build --profile release-static

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed (exit code: $LASTEXITCODE)" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}
