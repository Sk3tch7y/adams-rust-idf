# Rust ESP-IDF Project

This project is a Rust application targeting Espressif microcontrollers using the [ESP-IDF](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/) framework.  
It was created to act as a baseline for developing my personal projects using Rust on ESP devices.

## Features

- Written in Rust
- Message based architecture
- Exec Service for task management
- Easily extensible for your own IoT projects

## Prerequisites

- [Rust toolchain](https://rustup.rs/) (nightly recommended)
- [cargo-espflash](https://github.com/esp-rs/espflash) for flashing
- [espup](https://github.com/esp-rs/espup) for installing ESP-IDF
- Python 3 (required by ESP-IDF)
- Supported Espressif board (e.g., ESP32, ESP32-C3, ESP8266)
    - I have only tested a ESP32 board.

## Setup Instructions

### 1. Install Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
rustup target add xtensa-esp32-espidf
```

### 2. Install ESP-IDF

```sh
cargo install espup
espup install
```

Follow the instructions to set up the ESP-IDF environment variables. this may include running a script like "export-esp.sh"

### 3. Install cargo-espflash

```sh
cargo install cargo-espflash
```

### 4. Build the Project

```sh
cargo build
```

### 5. Flash to Device

Connect your ESP board and run:

```sh
 cargo espflash flash --baud 921600 -p {Your Serial Port} --monitor
```

Reduce the baud rate if you encounter issues.

## Extending the Project

To add your own functionality, you can create new modules and services within the `src/services` directory. Follow the existing structure to maintain consistency. New services can follow the template provided in `src/services/led_test.rs`. They should also be configured in the exec functionality located in `src/exec/exec.rs`.

new message types can be defined in `src/messages.rs` and handle them in your services.

For new services I recommend setting the stack size much higher than a typical C project as Rust tasks can use more significantly more stack space due to safety checks and abstractions. pushing values onto the heap can help reduce stack usage, as rust will manage the memory for you.
