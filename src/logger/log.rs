use log;
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Error,
}

pub fn log_init() {
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Trace);
}

#[allow(dead_code)]
pub struct Logger {
    log_level: LogLevel,
    service_name: String,
}
#[allow(dead_code)]
impl Logger {
    // TODO: Add service name initialization for param checking
    pub fn new(service_name: &str) -> Self {
        Self {
            // Default log level is Info
            log_level: LogLevel::Info,
            service_name: (*service_name).to_string(),
            // Adjust based on params if needed
        }
    }

    pub fn trace(&self, msg: &str) {
        log::trace!("TRACE | {}: {}", self.service_name, msg);
    }

    pub fn debug(&self, msg: &str) {
        log::debug!("DEBUG | {}: {}", self.service_name, msg);
    }

    pub fn info(&self, msg: &str) {
        log::info!("INFO  | {}: {}", self.service_name, msg);
    }

    pub fn error(&self, msg: &str) {
        log::error!("ERROR | {}: {}", self.service_name, msg);
    }
}
