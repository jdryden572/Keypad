#include <Bounce2.h>
#include <EEPROM.h>

const int NUM_KEYS = 6;
const int buttonPins[NUM_KEYS] = {23, 22, 0, 1, 2, 3};
const int ledPins[NUM_KEYS] = {20, 17, 16, 10, 9, 6};
const int ledIntensities[NUM_KEYS] = {30, 30, 30, 30, 30, 30};
const int debounceInterval = 10;

Bounce buttons[NUM_KEYS];

struct KeyCombo {
  int modifier;
  int key;
};

KeyCombo keyCombos[NUM_KEYS];

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
}

void storeKeyCombos() {
  for (int i = 0; i < NUM_KEYS; i++) {
    int address = i * 10;
    EEPROM.put(address, keyCombos[i]);
  }
}

void loadKeyCombos() {
  for (int i = 0; i < NUM_KEYS; i++) {
    int address = i * 10;
    EEPROM.get(address, keyCombos[i]);
  }
}

void loop() {
  for (int i = 0; i < NUM_KEYS; i++) {
    buttons[i].update();
    if (buttons[i].fell()) {
      analogWrite(ledPins[i], ledIntensities[i]);
      sendKeyCombo(keyCombos[i]);
    }
    else if (buttons[i].rose()) {
      analogWrite(ledPins[i], 0);
    }
  }

  handleSerial();
}

void handleSerial() {
  if (Serial.available() > 0) {
    char received = Serial.read();
    if (received == 'P') {
      sendKeyCombos();
    }
    else if (received == 'S') {
      setCombos();
    }
  }
}

void sendKeyCombos() {
  for (int i = 0; i < NUM_KEYS; i++) {
    Serial.write(lowByte(keyCombos[i].modifier));
    Serial.write(highByte(keyCombos[i].modifier));
    Serial.write(lowByte(keyCombos[i].key));
    Serial.write(highByte(keyCombos[i].key));
  }
}

void setCombos() {
  readKeyCombosFromSerial();
  storeKeyCombos();
  sendKeyCombos();
}

void readKeyCombosFromSerial() {
  const byte BUF_SIZE = NUM_KEYS * 4;
  char buf[BUF_SIZE];
  int readLen = Serial.readBytes(buf, BUF_SIZE);
  if (readLen == BUF_SIZE) {
    for (int i = 0; i < NUM_KEYS; i++) {
      int startIdx = 4 * i;
      int modifier = buf[startIdx + 1] << 8 | buf[startIdx];
      int key = buf[startIdx + 3] << 8 | buf[startIdx + 2];
      keyCombos[i] = KeyCombo {
        modifier,
        key
      };
    }
  }
}

void sendKeyCombo(KeyCombo combo) {
  if (combo.modifier != 0) {
    Keyboard.set_modifier(combo.modifier);
    Keyboard.send_now();
  }
  
  if (combo.key != 0) {
    Keyboard.set_key1(combo.key);
    Keyboard.send_now();
  }

  Keyboard.set_modifier(0);
  Keyboard.set_key1(0);
  Keyboard.send_now();
}
