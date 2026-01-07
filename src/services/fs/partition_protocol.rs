use crate::hardware::pins::Pin;
pub enum PartitionProtocol {
    SPI,   //MOSI, MISO, SCK, CS
    I2C,   //unsupported ATM
    USB,   //unsupported ATM, as ESP32 base chip doesnt have USB host capabilities
    SDMMC, //unsupported ATM
}

pub struct MountSet {
    pub pins: Vec<Pin>,
    pub protocol: PartitionProtocol,
}
