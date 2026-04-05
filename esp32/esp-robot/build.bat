@echo off
set PATH=%USERPROFILE%\.rustup\toolchains\esp\xtensa-esp-elf\bin;%PATH%
set PATH=%USERPROFILE%\.rustup\toolchains\esp\xtensa-esp32-elf-clang\esp-clang\bin;%PATH%
set LIBCLANG_PATH=%USERPROFILE%\.rustup\toolchains\esp\xtensa-esp32-elf-clang\esp-clang\bin\libclang.dll
cd /d "%~dp0esp-robot"
cargo build --release
