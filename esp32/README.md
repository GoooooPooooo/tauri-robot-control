# ESP32 Robot Car Controller

## Схема подключения

```
ESP32          L298N Motor Driver
-------        ------------------
GPIO 5   ----> ENA (PWM Left Motor)
GPIO 17  ----> IN1
GPIO 18  ----> IN2
GPIO 19  ----> IN3
GPIO 21  ----> IN4
GPIO 22  ----> ENB (PWM Right Motor)

L298N         Motors
----         ------
OUT1      ----> Left Motor +
OUT2      ----> Left Motor -
OUT3      ----> Right Motor +
OUT4      ----> Right Motor -

L298N         Power
----         -----
12V      ----> LiPo Battery (+)  (3S 11.1V)
GND      ----> Battery (-), ESP32 GND
5V       ----> (не используется, или для питания ESP32)
```

## Установка Arduino IDE для ESP32

### Шаг 1: Скачайте Arduino IDE
1. Скачайте с https://www.arduino.cc/en/software
2. Установите

### Шаг 2: Добавьте поддержку ESP32
1. Откройте Arduino IDE
2. Файл → Настройки
3. В поле "Дополнительные ссылки для менеджера плат" добавьте:
   ```
   https://raw.githubusercontent.com/espressif/arduino-esp32/gh-pages/package_esp32_index.json
   ```
4. Нажмите ОК
5. Инструменты → Плата → Менеджер плат
6. Найдите "ESP32" и установите

### Шаг 3: Выберите плату
1. Инструменты → Плата → ESP32 Arduino → "ESP32 Dev Module"

## Прошивка ESP32

### Шаг 1: Настройте код
Откройте `robot_car.ino` и измените:

```cpp
// IP адрес вашего ПК в локальной сети
const char* PC_IP = "192.168.1.100";  // ← ИЗМЕНИТЕ!

// WiFi настройки
const char* WIFI_SSID = "YOUR_WIFI_SSID";      // ← ИМЯ ВАШЕЙ СЕТИ
const char* WIFI_PASSWORD = "YOUR_WIFI_PASSWORD"; // ← ПАРОЛЬ СЕТИ
```

**Как узнать IP вашего ПК:**
1. Откройте командную строку (Win + R, введите `cmd`)
2. Введите `ipconfig`
3. Найдите "IPv4-адрес" (например, 192.168.1.100)

### Шаг 2: Подключите ESP32
1. Подключите ESP32 к компьютеру через USB
2. Выберите порт: Инструменты → Порт → COMX (где X - номер порта)

### Шаг 3: Загрузите код
1. Откройте `robot_car.ino` в Arduino IDE
2. Нажмите кнопку "Загрузить" (стрелка вправо →)
3. Дождитесь надписи "Загрузка завершена"

### Шаг 4: Проверьте в Serial Monitor
1. Инструменты → Serial Monitor (или Ctrl+Shift+M)
2. Установите скорость 115200
3. Вы должны увидеть:
   ```
   ========================================
     Robot Car ESP32 Controller
   ========================================
   Connecting to WiFi...
   WiFi Connected!
   ESP32 IP: 192.168.1.101
   ========================================
   ```

## Запуск сервера на ПК

### Шаг 1: Узнайте IP вашего ПК
```cmd
ipconfig
```
Найдите IPv4 адрес (например, 192.168.1.100)

### Шаг 2: Обновите IP в коде ESP32
Измените `PC_IP` на IP вашего ПК и перепрошейте.

### Шаг 3: Запустите приложение
```powershell
cd C:\projects\tauri-app
.\start.bat
```

В консоли увидите:
```
========================================
  HTTP Server for ESP32
========================================
  ESP32 connect to: http://<YOUR_PC_IP>:8080/command
  Or: http://localhost:8080/command
========================================
```

## Проверка работы

### Через браузер (для теста):
Откройте в браузере на ПК:
```
http://localhost:8080/command
```
Должно показать: `STOP`

### Через Serial Monitor ESP32:
В Arduino IDE → Serial Monitor (115200) вы должны видеть:
```
Command: STOP
```
Когда нажмете кнопку "Forward" в приложении, увидите:
```
Command: FORWARD
```

## Управление

### С компьютера:
- **Стрелки** или **WASD** - движение
- **Пробел** или **Escape** - стоп
- Кнопки на экране

### С ESP32:
ESP32 автоматически опрашивает сервер каждые 100мс и выполняет команды:
- `FORWARD` - вперед
- `BACKWARD` - назад
- `LEFT` - влево
- `RIGHT` - вправо
- `STOP` - стоп

## Возможные проблемы

### ESP32 не подключается к WiFi
- Проверьте имя и пароль сети
- Убедитесь, что WiFi 2.4GHz (ESP32 не поддерживает 5GHz)

### ESP32 не видит сервер
- Проверьте IP адрес ПК
- Проверьте firewall: Windows может блокировать порт 8080
  - Разрешить в брандмауэре: `netsh advfirewall firewall add rule name="Robot Server" dir=in action=allow protocol=tcp localport=8080`

### Моторы не работают
- Проверьте подключение L298N
- Проверьте питание (аккумулятор должен быть 7-12V)
- Убедитесь, что ESP32 и L298N имеют общий GND

## Альтернатива: PlatformIO

Если используете PlatformIO вместо Arduino IDE:

1. Установите PlatformIO VSCode extension
2. Создайте проект: PlatformIO → New Project → ESP32
3. Скопируйте содержимое `.ino` файла в `src/main.cpp`
4. Добавьте в `platformio.ini`:
   ```ini
   lib_deps = arduinojson
   ```
5. Загрузите: PlatformIO → Upload

## Материалы

- [ESP32 Arduino Core](https://github.com/espressif/arduino-esp32)
- [L298N datasheet](https://www.sparkfun.com/datasheets/Robotics/L298_H_Bridge.pdf)
- [WiFi библиотека ESP32](https://docs.espressif.com/projects/arduino-esp32/en/latest/api/wifi.html)
