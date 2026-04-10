# ESP32 Robot Car Control System (В разработке)

Control your ESP32 robot car from PC or smartphone using a Tauri desktop application.

## Project Structure

```
tauri-app/
├── src/                      # React frontend
│   ├── App.tsx              # Main UI component
│   └── styles.css           # CSS styles
├── src-tauri/               # Rust backend
│   └── src/main.rs          # HTTP server + Tauri commands
├── esp32/                   # ESP32 firmware
│   ├── esp-robot/           # esp-hal (no_std) version
│   ├── esp-robot-idf/       # esp-idf-sys (std) version
│   └── arduino/             # Arduino version
└── start.bat                # Launch script
```

## Quick Start

### 1. Start the Tauri App

```cmd
cd C:\projects\tauri-app
start.bat
```

The HTTP server will start on port 8080.

### 2. Flash ESP32 Firmware

#### Option A: Arduino (Recommended - has WiFi)
1. Open `esp32/arduino/robot_car/robot_car.ino` in Arduino IDE
2. Update WiFi credentials if needed
3. Update SERVER_IP to your PC's IP address
4. Flash to ESP32

#### Option B: Rust (esp-hal no_std)
```cmd
cd C:\projects\tauri-app\esp32\esp-robot
build.bat
espflash flash --monitor --chip esp32 --serial-port COM3
```

#### Option C: Rust with ESP-IDF (full std)
See `esp32/esp-robot-idf/INSTALL.md` for setup instructions.

### 3. Connect and Control

1. ESP32 will connect to your WiFi network
2. ESP32 polls the server every 100ms for commands
3. Use the Tauri app or keyboard to control the robot

## Control Options

### Desktop (PC)
- **Keyboard**: Arrow keys or WASD for direction
- **Space/Esc**: Stop
- **On-screen buttons**: Click or touch

### Mobile (Smartphone)
- Use on-screen buttons
- Works in browser (connect to same network)

## HTTP API

The ESP32 connects to:
```
http://<PC_IP>:8080/command
```

Response: Plain text command (`FORWARD`, `BACKWARD`, `LEFT`, `RIGHT`, `STOP`)

## WiFi Configuration

Update in ESP32 code:
```cpp
const char* ssid = "your_wifi_ssid";
const char* password = "your_wifi_password";
const char* serverIP = "192.168.50.19";  // Your PC's IP
```

## Motor Pins

| Signal | ESP32 GPIO |
|--------|------------|
| IN1    | GPIO26     |
| IN2    | GPIO27     |
| IN3    | GPIO14     |
| IN4    | GPIO12     |
| ENA    | GPIO25     |
| ENB    | GPIO33     |

## Troubleshooting

1. **ESP32 can't connect to WiFi**: Check SSID/password
2. **ESP32 can't reach server**: Check firewall settings on PC
3. **Server not responding**: Make sure port 8080 is open
4. **Commands not working**: Check the HTTP response format

## Building the Tauri App

```cmd
cd C:\projects\tauri-app
npm install
npm run tauri build
```
