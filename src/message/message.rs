use std::any::Any;
use std::sync::Arc;
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
#[allow(dead_code)]
pub enum MessageType {
    MidDummy = 0, //this must be first
    // TYPE, Description
    // System Control Messages
    MidSystemShutdown,
    MidSystemRestart,
    MidSystemPowerSourceChanged, // NA, System Power Source Changed

    //Rotary Encoder Control Messages
    MidRotaryEncoderChanged,     // NA, Rotary Encoder Value Changed
    MidRotaryEncoderPressed,     // NA, Rotary Encoder Button Pressed
    MidRotaryEncoderReleased,    // NA, Rotary Encoder Button Released
    MidRotaryEncoderClicked,     // NA, Rotary Encoder Button Clicked
    MidRotaryEncoderLongPressed, // NA, Rotary Encoder Button Long Pressed
    MidRotaryEncoderDoubleClick, // NA, Rotary Encoder Button Double Clicked

    //Display messages
    MidDisplayUpdate,        // NA, Update Display
    MidDisplayClear,         // NA, Clear Display
    MidDisplaySetBrightness, // NA, Set Display Brightness
    MidDisplaySetContrast,   // NA, Set Display Contrast

    // WIFI Messages
    MidWifiConnected,          // NA, WiFi Connected
    MidWifiDisconnected,       // NA, WiFi Disconnected
    MidWifiGotIp,              // NA, WiFi Got IP Address
    MidWifiScanResult,         // NA, WiFi Scan Result
    MidWifiAccessPointStarted, // NA, WiFi Access Point Started
    MidWifiAccessPointStopped, // NA, WiFi Access Point Stopped

    //Bluetooth Messages

    //for General use
    MidGeneralPurpose1,  // NA, General purpose message type 1
    MidGeneralPurpose2,  // NA, General purpose message type 2
    MidGeneralPurpose3,  // NA, General purpose message type 3
    MidGeneralPurpose4,  // NA, General purpose message type 4
    MidGeneralPurpose5,  // NA, General purpose message type 5
    MidGeneralPurpose6,  // NA, General purpose message type 6
    MidGeneralPurpose7,  // NA, General purpose message type 7
    MidGeneralPurpose8,  // NA, General purpose message type 8
    MidGeneralPurpose9,  // NA, General purpose message type 9
    MidGeneralPurpose10, // NA, General purpose message type 10

    MidCount, //This must be last
}

#[derive(Debug, Clone)]
pub struct Message {
    pub msg_type: MessageType,
    pub payload: Arc<dyn Any + Send + Sync>,
}
