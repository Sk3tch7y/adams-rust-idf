use super::message::Message;
use super::message::MessageType;
use crate::utils::time_utils::ms_to_ticks;

use esp_idf_hal::sys::{xQueueGenericCreate, xQueueGenericSend, xQueueReceive};
use esp_idf_sys::QueueHandle_t;
use std::collections::HashSet;
use std::ffi::c_void;
use std::sync::Arc;

type MessagePtr = *mut Message;

#[derive(Clone)]
pub struct Subscriber {
    message_queue: QueueHandle_t,
    subscriptions: HashSet<MessageType>,
}

unsafe impl Send for Subscriber {}
unsafe impl Sync for Subscriber {}
impl Subscriber {
    pub fn new() -> Self {
        let message_queue =
            unsafe { xQueueGenericCreate(10, std::mem::size_of::<MessagePtr>() as u32, 0) };
        assert!(!message_queue.is_null(), "Failed to create message queue");
        Self {
            message_queue,
            subscriptions: HashSet::new(),
        }
    }

    pub fn post(&self, msg: Arc<Message>) -> bool {
        if self.subscriptions.contains(&msg.msg_type) {
            let ptr = Arc::into_raw(msg);
            let res = unsafe {
                xQueueGenericSend(self.message_queue, &ptr as *const _ as *const c_void, 0, 0)
            };
            if res == 1 {
                true
            } else {
                unsafe {
                    drop(Arc::from_raw(ptr));
                }
                false
            }
        } else {
            false
        }
    }

    pub fn get(&self, timeout_ms: u32) -> Option<Arc<Message>> {
        let mut ptr: *const Message = std::ptr::null();
        let ticks = ms_to_ticks(timeout_ms);

        let res = unsafe {
            xQueueReceive(
                self.message_queue,
                &mut ptr as *mut _ as *mut std::ffi::c_void,
                ticks,
            )
        };

        if res == 1 && !ptr.is_null() {
            let arc_msg = unsafe { Arc::from_raw(ptr) };
            Some(arc_msg)
        } else {
            None
        }
    }

    pub fn subscribe(&mut self, msgtype: MessageType) {
        self.subscriptions.insert(msgtype);
    }

    pub fn unsubscribe(&mut self, msgtype: MessageType) {
        self.subscriptions.remove(&msgtype);
    }
}
