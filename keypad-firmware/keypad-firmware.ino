#include <Bounce2.h>
#include <EEPROM.h>

const int NUM_KEYS = 6;
const int buttonPins[NUM_KEYS] = {23, 22, 0, 1, 2, 3};
const int ledPins[NUM_KEYS] = {20, 17, 16, 10, 9, 6};
const int ledIntensities[NUM_KEYS] = {30, 30, 30, 30, 30, 30};
const int debounceInterval = 10;

Bounce buttons[NUM_KEYS] = {Bounce(), Bounce(), Bounce(), Bounce(), Bounce(), Bounce()};

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
      printCombos();
    }
  }
}

void printCombos() {
  for (int i = 0; i < NUM_KEYS; i++) {
    Serial.println(keyCombos[i].modifier - 0xE000);
    Serial.println(keyCombos[i].key - 0xF000);
    Serial.println();
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
