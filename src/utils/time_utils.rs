//TODO: make these functions configurable based on actual tick rate from ESP-IDF config
pub fn ms_to_ticks(ms: u32) -> u32 {
    // configTICK_RATE_HZ is usually 100Hz (10ms per tick), but check your ESP-IDF config!
    // ESP-IDF provides portTICK_PERIOD_MS as a macro in C, but not directly in Rust.
    // Here we assume 1 tick = 10ms (100Hz). Adjust as needed.
    return (ms + 9) / 10;
}
#[allow(dead_code)]
pub fn ticks_to_ms(ticks: u32) -> u32 {
    // Assuming 1 tick = 10ms (100Hz). Adjust as needed.
    return ticks * 10;
}
