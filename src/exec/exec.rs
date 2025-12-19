use crate::logger;
use crate::services;
use esp_idf_sys::*;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::ffi::c_void;
use std::ptr;
#[allow(dead_code)]
pub struct Task {
    task_handle: esp_idf_sys::TaskHandle_t,
    task_name: &'static str,
    task_func: unsafe extern "C" fn(*mut c_void) -> (),
    task_stack_size: usize,
    task_priority: u32,
    task_dependencies: Vec<&'static str>,
}

pub struct Exec {
    pub tasks: Vec<Task>,
    pub tasks_started: HashSet<&'static str>,
}

static EXEC_LOGGER: Lazy<logger::log::Logger> = Lazy::new(|| logger::log::Logger::new("exec"));

impl Exec {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            tasks_started: HashSet::new(),
        }
    }

    pub fn register(
        &mut self,
        task_name: &'static str,
        task_func: unsafe extern "C" fn(*mut c_void) -> (),
        task_stack_size: usize,
        task_priority: u32,
        task_dependencies: Vec<&'static str>,
    ) {
        self.tasks.push(Task {
            task_handle: ptr::null_mut(),
            task_name: task_name,
            task_func: task_func,
            task_stack_size: task_stack_size,
            task_priority: task_priority,
            task_dependencies: task_dependencies,
        });
    }

    fn register_tasks(&mut self) {
        self.register(
            "led_test",
            services::led_test::led_test_task_wrapper,
            4096,
            10,
            vec![],
        );
    }

    pub fn init(&mut self) {
        self.register_tasks();
    }
    pub fn exec(&mut self) {
        self.init();

        for task in &mut self.tasks {
            // TODO: Check dependencies before starting task
            // If dependencies are not met start them first
            // Tasks run an first available core, this can be changed to reserve a
            // core for mission critical tasks
            unsafe {
                xTaskCreatePinnedToCore(
                    Some(task.task_func),
                    task.task_name.as_ptr() as *const u8,
                    task.task_stack_size as u32,
                    std::ptr::null_mut(),
                    task.task_priority as u32,
                    &mut task.task_handle as *mut TaskHandle_t,
                    0x7FFFFFFF as i32,
                );
            }
            EXEC_LOGGER.info(
                format!(
                    "Starting task: {} with stack size {}",
                    task.task_name, task.task_stack_size
                )
                .as_str(),
            );
            self.tasks_started.insert(task.task_name);
        }
    }
}
