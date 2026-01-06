use crate::hardware::peripherals::PERIPHERALS;
use crate::logger;
use crate::message::message;
use crate::message::msg;
use esp_idf_hal::gpio::PinDriver;
use once_cell::sync::Lazy;

pub unsafe extern "C" fn led_test_task_wrapper(_arg: *mut core::ffi::c_void) {
    led_test_task();
}

static LED_TEST_LOGGER: Lazy<logger::log::Logger> =
    Lazy::new(|| logger::log::Logger::new("led_test"));

static mut MSG_QUEUE: msg::MsgQueueHandle = 0x7FFFFF;

fn turn_led_on(led: &mut PinDriver<esp_idf_hal::gpio::Gpio2, esp_idf_hal::gpio::Output>) {
    led.set_high().unwrap();
    LED_TEST_LOGGER.info("LED ON");
}

fn turn_led_off(led: &mut PinDriver<esp_idf_hal::gpio::Gpio2, esp_idf_hal::gpio::Output>) {
    led.set_low().unwrap();
    LED_TEST_LOGGER.info("LED OFF");
}

pub fn led_test_task() {
    //intialization
    //Create Queue
    unsafe {
        MSG_QUEUE = msg::create_msg_queue();
        msg::subscribe(MSG_QUEUE, message::MessageType::MidGeneralPurpose1);
    }

    // Create logger
    let mut peripherals_guard = PERIPHERALS.lock().unwrap();
    let peripherals = peripherals_guard.as_mut().unwrap();
    let mut led = PinDriver::output(&mut peripherals.pins.gpio2).unwrap();

    //core task loop
    loop {
        if let Some(msg) = unsafe { msg::get(MSG_QUEUE, 1000) } {
            let led_mode: bool = msg.payload.downcast_ref::<bool>().map_or(false, |b| *b);
            if led_mode {
                turn_led_on(&mut led);
            } else {
                turn_led_off(&mut led);
            }
        } else {
            LED_TEST_LOGGER.info("No message received within timeout");
        }
    }
}
