#include <Bounce2.h>
#include <EEPROM.h>

const int NUM_KEYS = 6;
const int buttonPins[NUM_KEYS] = {23, 22, 0, 1, 2, 3};
const int ledPins[NUM_KEYS] = {20, 17, 16, 10, 9, 6};
const int ledIntensity = 30;
const int debounceInterval = 10;

const char READ_KEYS = 'R';
const char WRITE_KEYS = 'W';
const char FLASH = 'F';
const char HELLO = 'H';
const char ACK = 'A';

Bounce buttons[NUM_KEYS];

struct KeyCombo {
  int modifier_one;
  int modifier_two;
  int key_one;
  int key_two;
};

KeyCombo keyCombos[NUM_KEYS];

unsigned long flashLEDsStart = 0;
byte flashingMask = 255;
bool flashingLEDs = false;
bool lightsOn = false;

void setup() {
  loadKeyCombos();
  Serial.begin(115200);
  
  for (int i = 0; i < NUM_KEYS; i++) {
    pinMode(buttonPins[i], INPUT_PULLUP);
    buttons[i] = Bounce();
    buttons[i].attach(buttonPins[i]);
    buttons[i].interval(debounceInterval);
    pinMode(ledPins[i], OUTPUT);
  }

  startFlashingLEDs();
}

void storeKeyCombos() {
  for (int i = 0; i < NUM_KEYS; i++) {
    int address = i * sizeof(KeyCombo);
    EEPROM.put(address, keyCombos[i]);
  }
}

void loadKeyCombos() {
  for (int i = 0; i < NUM_KEYS; i++) {
    int address = i * sizeof(KeyCombo);
    EEPROM.get(address, keyCombos[i]);
  }
}

void loop() {
  for (int i = 0; i < NUM_KEYS; i++) {
    buttons[i].update();
    if (buttons[i].fell()) {
      analogWrite(ledPins[i], ledIntensity);
      sendKeyCombo(keyCombos[i]);
    }
    else if (buttons[i].rose()) {
      analogWrite(ledPins[i], 0);
    }
  }

  updateFlashingLEDs();
  handleSerial();
}

void handleSerial() {
  if (Serial.available() > 0) {
    char received = Serial.read();
    switch (received) {
      case HELLO:
        Serial.write(ACK);
        break;
      case READ_KEYS:
        sendKeyCombos();
        break;
      case WRITE_KEYS:
        setCombos();
        break;
      case FLASH:
        flashKeys();
        break;
    }
  }
}

void sendKeyCombos() {
  for (int i = 0; i < NUM_KEYS; i++) {
    sendKey(keyCombos[i].modifier_one);
    sendKey(keyCombos[i].modifier_two);
    sendKey(keyCombos[i].key_one);
    sendKey(keyCombos[i].key_two);
  }
}

void sendKey(int key) {
  Serial.write(lowByte(key));
  Serial.write(highByte(key));
}

void setCombos() {
  readKeyCombosFromSerial();
  storeKeyCombos();
  sendKeyCombos();
}

void readKeyCombosFromSerial() {
  const byte KEY_BYTES = 8;
  const byte BUF_SIZE = NUM_KEYS * KEY_BYTES;
  char buf[BUF_SIZE];
  int readLen = Serial.readBytes(buf, BUF_SIZE);
  if (readLen == BUF_SIZE) {
    for (int i = 0; i < NUM_KEYS; i++) {
      int startIdx = KEY_BYTES * i;
      int modifier_one = buf[startIdx + 1] << 8 | buf[startIdx];
      int modifier_two = buf[startIdx + 3] << 8 | buf[startIdx + 2];
      int key_one = buf[startIdx + 5] << 8 | buf[startIdx + 4];
      int key_two = buf[startIdx + 7] << 8 | buf[startIdx + 6];
      keyCombos[i] = KeyCombo {
        modifier_one,
        modifier_two,
        key_one,
        key_two
      };
    }

    startFlashingLEDs();
  }
}

void flashKeys() {
  startFlashingLEDs();
  flashingMask = Serial.read();
  Serial.write(ACK);
}

void startFlashingLEDs() {
  allLights(false); // reset in case we're in the middle of a previous flash cycle
  flashingLEDs = true;
  flashLEDsStart = millis();
}

void updateFlashingLEDs() {
  if (!flashingLEDs) {
    return;
  }

  unsigned long elapsed = millis() - flashLEDsStart;
  
  if (elapsed > 1350) {
    flashingLEDs = false;
    flashingMask = 255;
    allLights(false);
  }
  else if (elapsed > 1000) {
    allLights(true);
  }
  else if (elapsed > 850) {
    allLights(false);
  }
  else if (elapsed > 500) {
    allLights(true);
  }
  else if (elapsed > 350) {
    allLights(false);
  }
  else {
    allLights(true);
  }
}

void allLights(bool on) {
  if (lightsOn != on) {
    lightsOn = on;
    int intensity = on ? ledIntensity : 0;
    for (int i = 0; i < NUM_KEYS; i++) {
      if ((flashingMask & (1 << i)) > 0) {
        analogWrite(ledPins[i], intensity);
      }
    }
  }
}

void sendKeyCombo(KeyCombo combo) {
  int modifier = combo.modifier_one;
  if (modifier != 0) {
    Keyboard.set_modifier(modifier);
    Keyboard.send_now();
  }
  
  if (combo.key_one != 0) {
    Keyboard.set_key1(combo.key_one);
    Keyboard.send_now();
  }

  if (combo.key_two != 0) {
    Keyboard.set_modifier(0);
    Keyboard.set_key1(0);
    Keyboard.send_now();
    Keyboard.set_modifier(combo.modifier_two);
    Keyboard.set_key1(combo.key_two);
    Keyboard.send_now();
  }

  Keyboard.set_modifier(0);
  Keyboard.set_key1(0);
  Keyboard.send_now();
}
