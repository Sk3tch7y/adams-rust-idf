use esp_idf_hal::peripherals::Peripherals;
use once_cell::sync::Lazy;
use std::boxed::Box;
use std::sync::Mutex;

pub static PERIPHERALS: Lazy<Mutex<Option<Box<Peripherals>>>> = Lazy::new(|| Mutex::new(None));

pub fn peripherals_init() {
    let mut guard = PERIPHERALS.lock().unwrap();
    if guard.is_none() {
        match Peripherals::take() {
            Ok(peripherals) => {
                *guard = Some(Box::new(peripherals));
                assert!(guard.is_some());
            }
            Err(_e) => {
                // if this happens, fault the program
                assert!(false);
            }
        }
    }
}
