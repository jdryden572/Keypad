#include <Bounce2.h>
#include <EEPROM.h>

const int NUM_KEYS = 6;
const int buttonPins[NUM_KEYS] = {23, 22, 0, 1, 2, 3};
const int ledPins[NUM_KEYS] = {20, 17, 16, 10, 9, 6};
const int ledIntensities[NUM_KEYS] = {30, 30, 30, 30, 30, 30};
const int debounceInterval = 10;

const char READ_KEYS = 'R';
const char WRITE_KEYS = 'W';
const char HELLO = 'H';
const char HELLO_ACK = 'A';

Bounce buttons[NUM_KEYS];

struct KeyCombo {
  int modifier_one;
  int modifier_two;
  int key_one;
  int key_two;
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
    switch (received) {
      case HELLO:
        Serial.write(HELLO_ACK);
        break;
      case READ_KEYS:
        sendKeyCombos();
        break;
      case WRITE_KEYS:
        setCombos();
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
  }
}

void sendKeyCombo(KeyCombo combo) {
  int modifier = combo.modifier_one | combo.modifier_two;
  if (modifier != 0) {
    Keyboard.set_modifier(modifier);
    Keyboard.send_now();
  }
  
  if (combo.key_one != 0) {
    Keyboard.set_key1(combo.key_one);
    Keyboard.send_now();
  }

  if (combo.key_two != 0) {
    Keyboard.set_key1(0);
    Keyboard.send_now();
    Keyboard.set_key1(combo.key_two);
    Keyboard.send_now();
  }

  Keyboard.set_modifier(0);
  Keyboard.set_key1(0);
  Keyboard.send_now();
}
