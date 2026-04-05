@echo off
REM Build and Flash ESP32 Robot
REM Usage: flash.bat [COM_PORT]
REM Example: flash.bat COM3

set PATH=%USERPROFACE%\.rustup\toolchains\esp\xtensa-esp-elf\bin;%PATH%
set PATH=%USERPROFILE%\.rustup\toolchains\esp\xtensa-esp32-elf-clang\esp-clang\bin;%PATH%
set LIBCLANG_PATH=%USERPROFILE%\.rustup\toolchains\esp\xtensa-esp32-elf-clang\esp-clang\bin\libclang.dll

cd /d "%~dp0esp-robot"

echo Building ESP32 Robot firmware...
cargo build --release

if errorlevel 1 (
    echo Build failed!
    exit /b 1
)

echo.
echo Build successful!
echo.

if "%1"=="" (
    echo Please specify COM port:
    echo   flash.bat COM3
    echo.
    echo Or run just to build:
    echo   build.bat
) else (
    echo Flashing to %1...
    espflash flash --monitor --chip esp32 --serial-port %1 target\xtensa-esp32-none-elf\release\esp-robot
)
