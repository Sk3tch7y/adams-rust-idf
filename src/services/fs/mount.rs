use crate::hardware;
use crate::logger;
use crate::services::fs::partition_protocol;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::config::DriverConfig;
use esp_idf_hal::spi::{config::Config, SpiDeviceDriver, SpiDriver};
use once_cell::sync::Lazy;
pub struct MountConfig {
    pub mount_point: partition_protocol::MountSet,
    pub format_if_mount_failed: bool,
    pub max_files: u8,
    pub allocation_unit_size: u32,
}

static MOUNT_LOGGER: Lazy<logger::log::Logger> =
    Lazy::new(|| logger::log::Logger::new("storage_mount"));

pub fn initialize_mount(config: MountConfig) {
    //intialize Pins for mount based on protocol
    // Only SPI currently supported
    match config.mount_point.protocol {
        partition_protocol::PartitionProtocol::SPI => {
            // Initialize SPI pins here using esp-idf-hal or esp-idf-sys
            MOUNT_LOGGER.info(
                format!(
                    "Initializing SPI Storage Mount on Pins {:?}{:?}{:?}{:?}",
                    config.mount_point.pins[0],
                    config.mount_point.pins[1],
                    config.mount_point.pins[2],
                    config.mount_point.pins[3]
                )
                .as_str(),
            );

            let sck = hardware::peripherals::get_esp_pin(config.mount_point.pins[0]);
            let miso = hardware::peripherals::get_esp_pin(config.mount_point.pins[1]);
            let mosi = hardware::peripherals::get_esp_pin(config.mount_point.pins[2]);
            let cs = hardware::peripherals::get_esp_pin(config.mount_point.pins[3]);

            let spi = SpiDriver::new(peripherals.spi2, sck, mosi, miso, &DriverConfig::default());

            MOUNT_LOGGER.info("SPI pins initialized for mounting.");
        }
        _ => {
            // Handle unsupported protocols
        }
    }
}
