use super::message::Message;
use super::message::MessageType;
use super::subscriber::Subscriber;
use crate::logger;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
pub type MsgQueueHandle = u32;

pub struct Msg {
    subscribers: Mutex<Vec<Subscriber>>,
    next_id: Mutex<MsgQueueHandle>,
}

static MSG_LOGGER: Lazy<logger::log::Logger> = Lazy::new(|| logger::log::Logger::new("msg"));
static MSG: OnceCell<Msg> = OnceCell::new();

pub fn init() {
    let msg: Msg = Msg {
        subscribers: Mutex::new(Vec::new()),
        next_id: Mutex::new(0 as MsgQueueHandle),
    };
    let err = { MSG.set(msg) };
    if err.is_err() {
        panic!("MSG already initialized");
    }
}

pub fn create_msg_queue() -> MsgQueueHandle {
    let mut id_lock = MSG.get().unwrap().next_id.lock().unwrap();
    let handler = Subscriber::new();
    *id_lock += 1;
    {
        MSG.get().unwrap().subscribers.lock().unwrap().push(handler);
    }
    return (*id_lock - 1) as MsgQueueHandle;
}

pub fn subscribe(handle: MsgQueueHandle, msg_type: MessageType) -> bool {
    MSG.get()
        .unwrap()
        .subscribers
        .lock()
        .unwrap()
        .get_mut(handle as usize)
        .map_or(false, |subscriber| {
            subscriber.subscribe(msg_type);
            return true;
        })
}
#[allow(dead_code)]
pub fn unsubscribe(handle: MsgQueueHandle, msg_type: MessageType) -> bool {
    MSG.get()
        .unwrap()
        .subscribers
        .lock()
        .unwrap()
        .get_mut(handle as usize)
        .map_or(false, |subscriber| {
            subscriber.unsubscribe(msg_type);
            return true;
        })
}

pub fn publish(msg: Message) {
    let mut subs: Vec<Subscriber>;
    {
        subs = MSG.get().unwrap().subscribers.lock().unwrap().clone();
    }
    MSG_LOGGER.trace(
        format!(
            "Publishing message of type {:?} to {} subscribers with Data {:?} bytes",
            msg.msg_type,
            subs.len(),
            msg.payload
        )
        .as_str(),
    );

    let arc_msg = Arc::new(msg);
    for subscriber in subs.iter_mut() {
        let sub = subscriber;
        sub.post(arc_msg.clone());
    }
}

pub fn get(handle: MsgQueueHandle, timeout_ms: u32) -> Option<Arc<Message>> {
    let mut subs: Vec<Subscriber>;
    {
        subs = MSG.get().unwrap().subscribers.lock().unwrap().clone()
    }
    let subscriber = subs.get_mut(handle as usize);
    if subscriber.is_none() {
        return None;
    }
    let subscriber = subscriber.unwrap();
    return subscriber.get(timeout_ms);
}
