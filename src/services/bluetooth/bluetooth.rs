pub struct BluetoothMessageRecievedEvent {
    pub conn_id: u16,
    pub attr_handle: u16,
    pub data: Vec<u8>,
}

impl BluetoothMessageRecievedEvent {
    pub fn new(conn_id: u16, attr_handle: u16, data: Vec<u8>) -> Self {
        BluetoothMessageRecievedEvent {
            conn_id,
            attr_handle,
            data,
        }
    }
}
