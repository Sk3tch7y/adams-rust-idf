use esp_idf_hal::delay::FreeRtos;
use log::info;
use std::sync::Arc;
mod exec;
mod hardware;
mod logger;
mod message;
mod services;
mod utils;

fn main() {
    // Link patches for ESP-IDF (required)
    esp_idf_sys::link_patches();

    // initialize peripherals
    hardware::peripherals::peripherals_init();

    // Initialize logging
    logger::log::log_init();
    info!("ESP32 Rust application starting!");

    // Initialize messages
    message::msg::init();

    //start Exec Service
    let mut exec = exec::exec::Exec::new();
    exec.exec();

    //create logger
    let main_logger = logger::log::Logger::new("main");
    loop {
        main_logger.info("Before LED ON publish");
        message::msg::publish(message::message::Message {
            msg_type: message::message::MessageType::MidGeneralPurpose1,
            payload: Arc::new(true),
        });
        main_logger.info("After LED ON publish");

        FreeRtos::delay_ms(1000);

        main_logger.info("Before LED OFF publish");
        message::msg::publish(message::message::Message {
            msg_type: message::message::MessageType::MidGeneralPurpose1,
            payload: Arc::new(false),
        });
        main_logger.info("After LED OFF publish");
        FreeRtos::delay_ms(1000);
    }
}
