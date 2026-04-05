@echo off
chcp 65001 >nul
title Robot Car Control

echo ========================================
echo   Robot Car Control - Starting...
echo ========================================
echo.

REM Set Rust path
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

REM Navigate to project directory
cd /d "%~dp0"

REM Start frontend in new window
echo [1/2] Starting frontend server...
start "Robot Frontend" cmd /c "npm run dev"
timeout /t 3 /nobreak >nul

REM Wait for frontend to be ready
echo Waiting for frontend to start...
:wait_loop
curl -s http://localhost:1420 >nul 2>&1
if errorlevel 1 (
    timeout /t 1 /nobreak >nul
    goto wait_loop
)

echo [2/2] Starting Rust backend...
echo.
echo ========================================
echo   Server ready! Opening application...
echo ========================================
echo.

REM Start Rust backend
cargo run --manifest-path src-tauri\Cargo.toml

pause
