# ESP32 Rust Build Script

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  ESP32 Rust Build Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Load ESP environment
. C:\Users\igore\export-esp.ps1

# Add target to stable
Write-Host "Adding RISCV target to stable..." -ForegroundColor Yellow
rustup target add riscv32imc-unknown-none-elf --toolchain stable

Write-Host ""
Write-Host "Building ESP32 project..." -ForegroundColor Green
Write-Host "Target: riscv32imc-unknown-none-elf (no_std)" -ForegroundColor Yellow
Write-Host ""

# Build with build-std for no_std
cargo build -Z build-std=core --target riscv32imc-unknown-none-elf

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Green
    Write-Host "  Build SUCCESS!" -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Red
    Write-Host "  Build FAILED!" -ForegroundColor Red
    Write-Host "========================================" -ForegroundColor Red
}
