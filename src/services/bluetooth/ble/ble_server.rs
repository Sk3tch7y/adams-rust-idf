use crate::logger;
use esp_idf_svc::ble::gatt_server::{self, Characteristic, Descriptor, GattServer, Service};
use esp_idf_svc::ble::Ble;
use esp_idf_sys;
use uuid::Uuid;

pub static BLE_LOGGER: logger::log::Logger = logger::log::Logger::new("ble_server");
static mut MSG_QUEUE: crate::message::msg::MsgQueueHandle = 0x7FFFFF;

pub unsafe extern "C" fn ble_task_wrapper(_arg: *mut core::ffi::c_void) {
    start_ble_server();
}

pub fn start_ble_server() -> Result<(), esp_idf_sys::EspError> {
    // Initialize msg queue
    let server_name = "RustBLEServer";
    // Initialize BLE
    let mut ble = Ble::new_default()?;

    let service_uuid = Uuid::new_v4();
    let char_uuid = Uuid::new_v4();

    // Create a GATT server
    let mut server = GattServer::new(&mut ble)?;

    // Add a service
    let service = Service::new(service_uuid, true);

    // Add a characteristic (readable and writable)
    let characteristic = Characteristic::new(
        char_uuid,
        Security::None,
        true, // readable
        true, // writable
        true, // notifiable
        vec![],
    );

    // Add the characteristic to the service
    let mut service = service.add_characteristic(characteristic);

    // Add the service to the server
    server.add_service(service);

    // Start advertising
    ble.advertise(server_name, &[])?;

    BLE_LOGGER.info(format!("GATT server running and advertising as {}", server_name).as_str());

    // Main loop
    loop {
        // Here you would handle events, e.g., read/write/notify

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
