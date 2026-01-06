use esp_idf_sys::{
    esp_vfs_littlefs_conf_t, esp_vfs_littlefs_register, esp_vfs_littlefs_unregister, ESP_OK,
};
use std::ffi::CString;
fn mount_sdcard_spi() {
    // SPI pin configuration
    let mut slot_config = sdmmc_slot_config_t {
        // Use SPI slot
        ..Default::default()
    };
    slot_config.flags = SDMMC_SLOT_FLAG_SPI;
    slot_config.gpio_miso = 19;
    slot_config.gpio_mosi = 23;
    slot_config.gpio_sck = 18;
    slot_config.gpio_cs = 5;

    // Host configuration for SPI
    let mut host = unsafe { sdmmc_host_t::default() };
    host.flags = SDMMC_HOST_FLAG_SPI;
    host.slot = SPI_HOST_DEVICE; // SPI2_HOST or SPI3_HOST, depending on your ESP32

    // Mount configuration
    let mount_config = esp_vfs_fat_sdmmc_mount_config_t {
        format_if_mount_failed: false,
        max_files: 5,
        allocation_unit_size: 16 * 1024,
    };

    let base_path = CString::new("/sdcard").unwrap();
    let mut card: *mut sdmmc_card_t = std::ptr::null_mut();

    let ret = unsafe {
        esp_vfs_fat_sdmmc_mount(
            base_path.as_ptr(),
            &host,
            &slot_config,
            &mount_config,
            &mut card,
        )
    };

    if ret != ESP_OK {
        println!("Failed to mount SD card: {}", ret);
        return;
    }

    // Use Rust's std::fs APIs
    std::fs::write("/sdcard/hello.txt", b"Hello SD Card!\n").unwrap();

    // Unmount when done
    unsafe { esp_vfs_fat_sdmmc_unmount(base_path.as_ptr(), card) };
}
