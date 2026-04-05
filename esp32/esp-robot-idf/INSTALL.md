# ESP-IDF Installation Guide (Windows)

## Option 1: Windows Installer (Recommended)

1. Download the ESP-IDF installer from:
   https://github.com/espressif/idf-windows-setup/releases

2. Run the installer (`esp-idf-installer-*-web-setup.exe`)

3. Follow the installation wizard:
   - Choose "Download ESP-IDF" or "Use existing repository"
   - Select ESP32 support
   - Install to default location (`C:\Users\<username>\.espressif`)

4. After installation, find the "ESP-IDF Command Prompt" shortcut

## Option 2: Manual Setup

1. Install prerequisites:
   - Python 3.8+ (from Microsoft Store or python.org)
   - Git for Windows
   - CMake (optional, for some builds)

2. Clone ESP-IDF:
   ```cmd
   git clone --recursive --depth 1 https://github.com/espressif/esp-idf.git C:\Users\igore\.espressif\esp-idf
   ```

3. Install Python dependencies:
   ```cmd
   cd C:\Users\igore\.espressif\esp-idf
   python -m pip install -r requirements.txt
   ```

4. Install ESP-IDF tools:
   ```cmd
   .\install.ps1 esp32
   ```

## After Installation

Set up environment variables by running:
```cmd
C:\Users\igore\.espressif\esp-idf\export.ps1
```

Or add to PATH manually:
- `C:\Users\igore\.espressif\esp-idf\tools`
- `C:\Users\igore\.espressif\tools\cmake\bin`
- `C:\Users\igore\.espressif\tools\ninja\bin`

## Quick Test

1. Open ESP-IDF Command Prompt
2. Run:
   ```cmd
   idf.py --version
   ```
   Should show ESP-IDF version

## Building the Rust Project

After ESP-IDF is set up, build the esp-robot-idf project:

```cmd
cd C:\projects\tauri-app\esp32\esp-robot-idf
idf.py set-target esp32
idf.py build
idf.py -p COM3 flash monitor
```

Replace `COM3` with your actual COM port.
