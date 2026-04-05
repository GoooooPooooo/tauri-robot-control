@echo off
chcp 65001 >nul
title ESP32 Rust Build

echo ========================================
echo   ESP32 Rust Build Script
echo ========================================
echo.

REM Load ESP environment variables
powershell -ExecutionPolicy Bypass -File C:\Users\igore\export-esp.ps1

echo.
echo Building ESP32 project (no_std)...
echo Target: xtensa-esp32-none-elf
echo.

cargo build

echo.
pause
