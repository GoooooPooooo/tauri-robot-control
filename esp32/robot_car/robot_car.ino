#include <WiFi.h>
#include <HTTPClient.h>
#include <ArduinoJson.h>

// ========== НАСТРОЙКИ ==========
// IP адрес вашего ПК (запустите ipconfig и найдите IPv4)
const char* PC_IP = "192.168.1.100";  // ИЗМЕНИТЕ НА IP ВАШЕГО ПК

// WiFi настройки
const char* WIFI_SSID = "YOUR_WIFI_SSID";      // ИМЯ СЕТИ
const char* WIFI_PASSWORD = "YOUR_WIFI_PASSWORD"; // ПАРОЛЬ СЕТИ

// Пин управления двигателями (L298N)
#define ENA 5   // PWM левый двигатель
#define IN1 17  // Направление левый
#define IN2 18  // Направление левый
#define IN3 19  // Направление правый
#define IN4 21  // Направление правый
#define ENB 22  // PWM правый двигатель

// Скорость двигателей (0-255)
#define MOTOR_SPEED 150

// URL сервера
const char* SERVER_URL = "http://192.168.1.100:8080/command";

// Интервал опроса (мс)
const int POLL_INTERVAL = 100;
// =================================

HTTPClient http;
unsigned long lastPoll = 0;
String lastCommand = "STOP";

void setup() {
  Serial.begin(115200);
  
  // Настройка пинов двигателей
  pinMode(ENA, OUTPUT);
  pinMode(IN1, OUTPUT);
  pinMode(IN2, OUTPUT);
  pinMode(IN3, OUTPUT);
  pinMode(IN4, OUTPUT);
  pinMode(ENB, OUTPUT);
  
  // Остановить двигатели
  stopMotors();
  
  Serial.println();
  Serial.println("========================================");
  Serial.println("  Robot Car ESP32 Controller");
  Serial.println("========================================");
  
  // Подключение к WiFi
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
  Serial.print("Connecting to WiFi");
  
  int attempts = 0;
  while (WiFi.status() != WL_CONNECTED && attempts < 30) {
    delay(500);
    Serial.print(".");
    attempts++;
  }
  
  if (WiFi.status() == WL_CONNECTED) {
    Serial.println();
    Serial.println("WiFi Connected!");
    Serial.print("ESP32 IP: ");
    Serial.println(WiFi.localIP());
    Serial.print("Server URL: ");
    Serial.println(SERVER_URL);
  } else {
    Serial.println();
    Serial.println("WiFi FAILED! Retrying...");
  }
  
  Serial.println("========================================");
  Serial.println("Starting motor control loop...");
  Serial.println("========================================");
}

void loop() {
  unsigned long currentMillis = millis();
  
  // Опрос сервера каждые POLL_INTERVAL мс
  if (currentMillis - lastPoll >= POLL_INTERVAL) {
    lastPoll = currentMillis;
    pollServer();
  }
}

void pollServer() {
  if (WiFi.status() != WL_CONNECTED) {
    Serial.println("WiFi disconnected! Reconnecting...");
    WiFi.reconnect();
    delay(1000);
    return;
  }
  
  http.begin(SERVER_URL);
  http.setTimeout(1000);
  
  int httpCode = http.GET();
  
  if (httpCode == 200) {
    String payload = http.getString();
    payload.trim();
    
    // Убираем возможные символы новой строки
    payload.replace("\n", "");
    payload.replace("\r", "");
    
    if (payload.length() > 0 && payload != lastCommand) {
      lastCommand = payload;
      executeCommand(payload);
    }
  } else if (httpCode == -1) {
    // Ошибка подключения
    Serial.println("Connection error - server may not be running");
  } else {
    Serial.print("HTTP Error: ");
    Serial.println(httpCode);
  }
  
  http.end();
}

void executeCommand(String cmd) {
  Serial.print("Command: ");
  Serial.println(cmd);
  
  if (cmd == "FORWARD") {
    moveForward();
  } else if (cmd == "BACKWARD") {
    moveBackward();
  } else if (cmd == "LEFT") {
    turnLeft();
  } else if (cmd == "RIGHT") {
    turnRight();
  } else if (cmd == "STOP") {
    stopMotors();
  } else {
    Serial.print("Unknown command: ");
    Serial.println(cmd);
    stopMotors();
  }
}

void moveForward() {
  analogWrite(ENA, MOTOR_SPEED);
  analogWrite(ENB, MOTOR_SPEED);
  digitalWrite(IN1, HIGH);
  digitalWrite(IN2, LOW);
  digitalWrite(IN3, HIGH);
  digitalWrite(IN4, LOW);
}

void moveBackward() {
  analogWrite(ENA, MOTOR_SPEED);
  analogWrite(ENB, MOTOR_SPEED);
  digitalWrite(IN1, LOW);
  digitalWrite(IN2, HIGH);
  digitalWrite(IN3, LOW);
  digitalWrite(IN4, HIGH);
}

void turnLeft() {
  analogWrite(ENA, MOTOR_SPEED / 2);
  analogWrite(ENB, MOTOR_SPEED);
  digitalWrite(IN1, LOW);
  digitalWrite(IN2, HIGH);
  digitalWrite(IN3, HIGH);
  digitalWrite(IN4, LOW);
}

void turnRight() {
  analogWrite(ENA, MOTOR_SPEED);
  analogWrite(ENB, MOTOR_SPEED / 2);
  digitalWrite(IN1, HIGH);
  digitalWrite(IN2, LOW);
  digitalWrite(IN3, LOW);
  digitalWrite(IN4, HIGH);
}

void stopMotors() {
  digitalWrite(IN1, LOW);
  digitalWrite(IN2, LOW);
  digitalWrite(IN3, LOW);
  digitalWrite(IN4, LOW);
  analogWrite(ENA, 0);
  analogWrite(ENB, 0);
}
