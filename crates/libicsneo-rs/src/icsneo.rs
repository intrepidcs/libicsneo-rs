//! native safe [libicsneo_sys](libicsneo_sys) functions
use libicsneo_sys::*;
use std::ffi::{CStr, CString};

use crate::types::*;

/// Find all Intrepid devices. Returns a Result of Vec<[NeoDevice](NeoDevice)>.
///
/// Returns [Error::NoDevicesFound](Error::NoDevicesFound) when no devices are present
///
/// See [icsneo_findAllDevices()](libicsneo_sys::icsneo_findAllDevices) for more details.
///
///
/// Example:
/// ```
///
/// use icsneo;
///
/// let devices = icsneo::find_all_devices().unwrap();
///
/// ```
pub fn find_all_devices() -> Result<Vec<NeoDevice>> {
    // Get the device count
    let device_count = unsafe {
        let mut device_count = 0;
        icsneo_findAllDevices(std::ptr::null_mut(), &mut device_count);
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {}
        };
        device_count
    };
    let mut devices = Vec::<neodevice_t>::new();
    devices.resize(device_count, neodevice_t::default());
    unsafe {
        let mut device_count = device_count;
        icsneo_findAllDevices(devices.as_mut_ptr() as *mut neodevice_t, &mut device_count);
        let event = get_last_error();
        match event {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {}
        };
    }
    let devices: Vec<NeoDevice> = devices.iter().map(|d| NeoDevice { 0: *d }).collect();
    Ok(devices)
}

/// Frees all unconnected devices. See [icsneo_freeUnconnectedDevices()](libicsneo_sys::icsneo_freeUnconnectedDevices) for more details
///
/// TODO: Description here
pub fn free_unconnected_devices() -> Result<()> {
    // extern void DLLExport icsneo_freeUnconnectedDevices();
    unsafe {
        icsneo_freeUnconnectedDevices();
    };
    Ok(())
}

/// Converts a serial number integer to a string. See [icsneo_serialNumToString()](libicsneo_sys::icsneo_serialNumToString) for more details
///
/// TODO: Description here
pub fn serial_num_to_string(num: u32) -> Result<String> {
    // extern bool DLLExport icsneo_serialNumToString(uint32_t num, char* str, size_t* count);

    // Grab the length needed
    let mut count = 0usize;
    let success = unsafe { icsneo_serialNumToString(num, std::ptr::null_mut(), &mut count) };
    // icsneo_serialNumToString returns false when we query for the str length.
    if success {
        return Err(Error::CriticalError(
            "icsneo_serialNumToString() failed to query length!".to_string(),
        ));
    }
    // Need to account for the null terminator to prevent OBOE
    count += 1;
    let mut buffer: Vec<i8> = vec![0; count as usize];
    let success = unsafe { icsneo_serialNumToString(num, buffer.as_mut_ptr(), &mut count) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_serialNumToString() failed to convert!".to_string(),
                ))
            }
        };
    }
    // Convert the CStr to a String on success
    unsafe {
        return match CStr::from_ptr(buffer.as_mut_ptr()).to_owned().to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => {
                let msg = format!("Failed to convert serial number buffer to CString: {e}");
                Err(Error::CriticalError(msg))
            }
        };
    };
}

/// Converts a serial number string to a base10 integer. See [icsneo_serialStringToNum()](libicsneo_sys::icsneo_serialStringToNum) for more details
///
/// TODO: Description here
pub fn serial_string_to_num(serial_str: &str) -> u32 {
    // extern uint32_t DLLExport icsneo_serialStringToNum(const char* str);
    let serial = CString::new(serial_str).unwrap();
    unsafe { icsneo_serialStringToNum(serial.as_ptr()) }
}

/// Returns the neoevent_t if an error occurred or None if none. See [icsneo_getLastError()](libicsneo_sys::icsneo_getLastError) for more details
///
/// TODO: Description here
pub fn get_last_error() -> Option<NeoEvent> {
    let mut neo_event = NeoEvent::default();
    unsafe {
        if icsneo_getLastError(&mut neo_event.0) {
            Some(neo_event)
        } else {
            None
        }
    }
}

/// See [icsneo_isValidNeoDevice()](libicsneo_sys::icsneo_isValidNeoDevice) for more details.
pub fn is_valid_neodevice(device: &NeoDevice) -> Result<bool> {
    // extern bool DLLExport icsneo_isValidNeoDevice(const neodevice_t* device);
    let is_valid = unsafe { icsneo_isValidNeoDevice(&device.0) };
    match get_last_error() {
        Some(e) => Err(Error::ErrorOccurred(e)),
        None => Ok(is_valid),
    }
}

/// Opens a neo device. See [icsneo_openDevice()](libicsneo_sys::icsneo_openDevice) for more details
///
/// TODO: Description here
pub fn open_device(device: &NeoDevice) -> Result<()> {
    // extern bool DLLExport icsneo_openDevice(const neodevice_t* device);
    let success = unsafe { icsneo_openDevice(&device.0) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "Bug() get_last_error() should have had an error".to_string(),
                ))
            }
        };
    }
    Ok(())
}

/// Closes a neo device. See [icsneo_closeDevice()](libicsneo_sys::icsneo_closeDevice) for more details
///
/// TODO: Description here
pub fn close_device(device: &NeoDevice) -> Result<()> {
    // extern bool DLLExport icsneo_closeDevice(const neodevice_t* device);
    let success = unsafe { icsneo_closeDevice(&device.0) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "Bug: get_last_error() should have had errors.".to_string(),
                ))
            }
        };
    }
    Ok(())
}

/// Checks to see if a neo device is open. See [icsneo_isOpen()](libicsneo_sys::icsneo_isOpen) for more details
///
/// TODO: Description here
pub fn is_open(device: &NeoDevice) -> Result<bool> {
    // extern bool DLLExport icsneo_isOpen(const neodevice_t* device);
    let success = unsafe { icsneo_isOpen(&device.0) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => (),
        };
    }
    Ok(success)
}

/// Goes online with a neo device. See [icsneo_goOnline()](libicsneo_sys::icsneo_goOnline) for more details
///
/// TODO: Description here
pub fn go_online(device: &NeoDevice) -> Result<()> {
    // extern bool DLLExport icsneo_goOnline(const neodevice_t* device);
    let success = unsafe { icsneo_goOnline(&device.0) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => return Err(Error::CriticalError("Couldn't go online".to_string())),
        };
    }
    Ok(())
}

/// Goes offline with a neo device. See [icsneo_goOffline()](libicsneo_sys::icsneo_goOffline) for more details
///
/// TODO: Description here
pub fn go_offline(device: &NeoDevice) -> Result<()> {
    // extern bool DLLExport icsneo_goOffline(const neodevice_t* device);
    let success = unsafe { icsneo_goOffline(&device.0) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => return Err(Error::CriticalError("Couldn't go offline".to_string())),
        };
    }
    Ok(())
}

/// Checks if the neo device is online. See [icsneo_isOnline()](libicsneo_sys::icsneo_isOnline) for more details
///
/// TODO: Description here
pub fn is_online(device: &NeoDevice) -> Result<bool> {
    // extern bool DLLExport icsneo_isOnline(const neodevice_t* device);
    let success = unsafe { icsneo_isOnline(&device.0) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {}
        };
    }
    Ok(success)
}

/// See [icsneo_enableMessagePolling()](libicsneo_sys::icsneo_enableMessagePolling) for more details
///
/// TODO: Description here
pub fn enable_message_polling(device: &NeoDevice) -> bool {
    unsafe { icsneo_enableMessagePolling(&device.0) }
}

/// See [icsneo_disableMessagePolling()](libicsneo_sys::icsneo_disableMessagePolling) for more details
///
/// TODO: Description here
pub fn disable_message_polling(device: &NeoDevice) -> bool {
    unsafe { icsneo_disableMessagePolling(&device.0) }
}

/// See [icsneo_isMessagePollingEnabled()](libicsneo_sys::icsneo_isMessagePollingEnabled) for more details
///
/// TODO: Description here
pub fn is_message_polling_enabled(device: &NeoDevice) -> bool {
    unsafe { icsneo_isMessagePollingEnabled(&device.0) }
}

/// See [icsneo_getMessages()](libicsneo_sys::icsneo_getMessages) for more details
///
/// TODO: Description here
pub fn get_messages(device: &NeoDevice, timeout: u64) -> Result<Vec<NeoMessage>> {
    // extern bool DLLExport icsneo_getMessages(const neodevice_t* device, neomessage_t* messages, size_t* items, uint64_t timeout);
    let mut count: usize = 0;
    let success =
        unsafe { icsneo_getMessages(&device.0, std::ptr::null_mut(), &mut count, timeout) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "Bug() get_last_error() should have had an error".to_string(),
                ))
            }
        };
    }
    // Initialize the messages
    let mut messages = Vec::with_capacity(count as usize);
    for _ in 0..count {
        messages.push(NeoMessage::default());
    }
    // Grab the messages
    let success = unsafe {
        icsneo_getMessages(
            &device.0,
            messages.as_mut_ptr() as *mut neomessage_t,
            &mut count,
            timeout,
        )
    };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "Bug() get_last_error() should have had an error".to_string(),
                ))
            }
        };
    }
    Ok(messages)
}

/// Returns message limit or Error::DeviceInvalid
/// See [icsneo_getPollingMessageLimit()](libicsneo_sys::icsneo_getPollingMessageLimit) for more details
///
/// TODO: Description here
pub fn get_polling_message_limit(device: &NeoDevice) -> Result<i32> {
    let count = unsafe { icsneo_getPollingMessageLimit(&device.0) };
    if count == -1 {
        return Err(Error::DeviceInvalid);
    };
    Ok(count)
}

/// Sets the message limit or Error::ErrorOccurred
/// See [icsneo_setPollingMessageLimit()](libicsneo_sys::icsneo_setPollingMessageLimit) for more details
///
/// TODO: Description here
pub fn set_polling_message_limit(device: &NeoDevice, message_count: usize) -> Result<()> {
    let success = unsafe { icsneo_setPollingMessageLimit(&device.0, message_count) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "Bug: get_last_error() should have had errors".to_string(),
                ))
            }
        };
    }
    Ok(())
}
/// See [icsneo_transmit()](libicsneo_sys::icsneo_transmit) for more details
///
/// TODO: Description here
pub fn transmit(device: &NeoDevice, message: &NeoMessage) -> Result<()> {
    let success = unsafe { icsneo_transmit(&device.0, &message.0) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "Bug: get_last_error() should have had errors".to_string(),
                ))
            }
        };
    }
    Ok(())
}

/// See [icsneo_transmitMessages()](libicsneo_sys::icsneo_transmitMessages) for more details
///
/// TODO: Description here
pub fn transmit_messages(device: &NeoDevice, messages: Vec<NeoMessage>) -> Result<()> {
    let success = unsafe {
        icsneo_transmitMessages(
            &device.0,
            messages.as_ptr() as *mut neomessage_t,
            messages.len() as usize,
        )
    };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "Bug: get_last_error() should have had errors".to_string(),
                ))
            }
        };
    }
    Ok(())
}

/// See [icsneo_describeDevice()](libicsneo_sys::icsneo_describeDevice) for more details
///
/// TODO: Description here
pub fn describe_device(device: &NeoDevice) -> Result<String> {
    let mut count = 0usize;
    let success = unsafe { icsneo_describeDevice(&device.0, std::ptr::null_mut(), &mut count) };
    // icsneo_describeDevice returns false when we query for the str length.
    if success {
        return Err(Error::CriticalError(
            "icsneo_serialNumToString() failed to query length!".to_string(),
        ));
    }
    // Need to account for the null terminator to prevent OBOE
    count += 1;
    let mut buffer: Vec<i8> = vec![0; count as usize];
    let success = unsafe { icsneo_describeDevice(&device.0, buffer.as_mut_ptr(), &mut count) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_describeDevice() failed!".to_string(),
                ))
            }
        };
    }
    // Convert the CStr to a String on success
    unsafe {
        return match CStr::from_ptr(buffer.as_mut_ptr()).to_owned().to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => {
                let msg = format!("Failed to device description buffer to CString: {e}");
                Err(Error::CriticalError(msg))
            }
        };
    };
}

/// See [icsneo_getNetworkByNumber()](libicsneo_sys::icsneo_getNetworkByNumber) for more details
///
/// TODO: Description here
pub fn get_network_by_number(
    device: &NeoDevice,
    neo_net_type: neonettype_t,
    number: u32,
) -> neonetid_t {
    unsafe { icsneo_getNetworkByNumber(&device.0, neo_net_type, number) }
}

/// See [icsneo_getProductName()](libicsneo_sys::icsneo_getProductName) for more details
///
/// TODO: Description here
pub fn get_product_name(device: &NeoDevice) -> Result<String> {
    let mut count = 0usize;
    let success = unsafe { icsneo_getProductName(&device.0, std::ptr::null_mut(), &mut count) };
    // icsneo_describeDevice returns false when we query for the str length.
    if success {
        return Err(Error::CriticalError(
            "icsneo_serialNumToString() failed to query length!".to_string(),
        ));
    }
    // Need to account for the null terminator to prevent OBOE
    count += 1;
    let mut buffer: Vec<i8> = vec![0; count as usize];
    let success = unsafe { icsneo_getProductName(&device.0, buffer.as_mut_ptr(), &mut count) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_getProductName() failed!".to_string(),
                ))
            }
        };
    }
    // Convert the CStr to a String on success
    unsafe {
        return match CStr::from_ptr(buffer.as_mut_ptr()).to_owned().to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => {
                let msg = format!("Failed to device description buffer to CString: {e}");
                Err(Error::CriticalError(msg))
            }
        };
    };
}

/// See [icsneo_getProductNameForType()](libicsneo_sys::icsneo_getProductNameForType) for more details
///
/// TODO: Description here
pub fn get_product_name_for_type(device: devicetype_t) -> Result<String> {
    let mut count = 0usize;
    let success = unsafe { icsneo_getProductNameForType(device, std::ptr::null_mut(), &mut count) };
    // icsneo_describeDevice returns false when we query for the str length.
    if success {
        return Err(Error::CriticalError(
            "icsneo_serialNumToString() failed to query length!".to_string(),
        ));
    }
    // Need to account for the null terminator to prevent OBOE
    count += 1;
    let mut buffer: Vec<i8> = vec![0; count as usize];
    let success = unsafe { icsneo_getProductNameForType(device, buffer.as_mut_ptr(), &mut count) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_getProductNameForType() failed!".to_string(),
                ))
            }
        };
    }
    // Convert the CStr to a String on success
    unsafe {
        return match CStr::from_ptr(buffer.as_mut_ptr()).to_owned().to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => {
                let msg = format!("Failed to device description buffer to CString: {e}");
                Err(Error::CriticalError(msg))
            }
        };
    };
}

/// See [icsneo_getVersion()](libicsneo_sys::icsneo_getVersion) for more details
///
/// TODO: Description here
pub fn get_version() -> NeoVersion {
    // extern neoversion_t DLLExport icsneo_getVersion(void);
    unsafe {
        NeoVersion {
            0: icsneo_getVersion(),
        }
    }
}

/// See [icsneo_getBaudrate()](libicsneo_sys::icsneo_getBaudrate) for more details
///
/// TODO: Description here
pub fn get_baudrate(device: &NeoDevice, netid: neonetid_t) -> i64 {
    // extern int64_t DLLExport icsneo_getBaudrate(const neodevice_t* device, neonetid_t netid);
    unsafe { icsneo_getBaudrate(&device.0, netid) }
}

/// See [icsneo_setBaudrate()](libicsneo_sys::icsneo_setBaudrate) for more details
///
/// TODO: Description here
pub fn set_baudrate(device: &NeoDevice, netid: neonetid_t, new_baudrate: i64) -> bool {
    // extern int64_t DLLExport icsneo_getBaudrate(const neodevice_t* device, neonetid_t netid);
    unsafe { icsneo_setBaudrate(&device.0, netid, new_baudrate) }
}

/// See [icsneo_getFDBaudrate()](libicsneo_sys::icsneo_getFDBaudrate) for more details
///
/// TODO: Description here
pub fn get_fd_baudrate(device: &NeoDevice, netid: neonetid_t) -> i64 {
    // extern int64_t DLLExport icsneo_getFDBaudrate(const neodevice_t* device, neonetid_t netid);
    unsafe { icsneo_getFDBaudrate(&device.0, netid) }
}

/// See [icsneo_setFDBaudrate()](libicsneo_sys::icsneo_setFDBaudrate) for more details
///
/// TODO: Description here
pub fn set_fd_baudrate(device: &NeoDevice, netid: neonetid_t, new_baudrate: i64) -> bool {
    // extern int64_t DLLExport icsneo_setFDBaudrate(const neodevice_t* device, neonetid_t netid);
    unsafe { icsneo_setFDBaudrate(&device.0, netid, new_baudrate) }
}

/// See [icsneo_setWriteBlocks()](libicsneo_sys::icsneo_setWriteBlocks) for more details
///
/// TODO: Description here
pub fn set_write_blocks(device: &NeoDevice, blocks: bool) {
    // extern void DLLExport icsneo_setWriteBlocks(const neodevice_t* device, bool blocks);
    unsafe {
        icsneo_setWriteBlocks(&device.0, blocks);
    }
}

/// See [icsneo_getEvents()](libicsneo_sys::icsneo_getEvents) for more details
///
/// TODO: Description here
pub fn get_events() -> Result<Vec<NeoEvent>> {
    // extern bool DLLExport icsneo_getEvents(neoevent_t* events, size_t* size);
    let mut size: usize = 0;
    let success = unsafe { icsneo_getEvents(std::ptr::null_mut(), &mut size) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_getEvents() failed!".to_string(),
                ))
            }
        };
    }
    let mut events = Vec::with_capacity(size as usize);
    for _ in 0..size {
        events.push(NeoEvent::default());
    }
    let success = unsafe { icsneo_getEvents(events.as_mut_ptr() as *mut _, &mut size) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_getEvents() failed!".to_string(),
                ))
            }
        };
    }
    Ok(events)
}

/// See [icsneo_getDeviceEvents()](libicsneo_sys::icsneo_getDeviceEvents) for more details
///
/// TODO: Description here
pub fn get_device_events(device: &NeoDevice) -> Result<Vec<NeoEvent>> {
    // extern bool DLLExport icsneo_getDeviceEvents(const neodevice_t* device, neoevent_t* events, size_t* size);
    let mut size: usize = 0;
    let success = unsafe { icsneo_getDeviceEvents(&device.0, std::ptr::null_mut(), &mut size) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_getDeviceEvents() failed!".to_string(),
                ))
            }
        };
    }
    let mut events = Vec::with_capacity(size as usize);
    for _ in 0..size {
        events.push(NeoEvent::default());
    }
    let success =
        unsafe { icsneo_getDeviceEvents(&device.0, events.as_mut_ptr() as *mut _, &mut size) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_getDeviceEvents() failed!".to_string(),
                ))
            }
        };
    }
    Ok(events)
}

/// See [icsneo_discardAllEvents()](libicsneo_sys::icsneo_discardAllEvents) for more details
///
/// TODO: Description here
pub fn discard_all_events() {
    // extern void DLLExport icsneo_discardAllEvents(void);
    unsafe {
        icsneo_discardAllEvents();
    };
}

/// See [icsneo_discardDeviceEvents()](libicsneo_sys::icsneo_discardDeviceEvents) for more details
///
/// TODO: Description here
pub fn discard_all_device_events(device: &NeoDevice) {
    // extern void DLLExport icsneo_discardDeviceEvents(const neodevice_t* device);
    unsafe {
        icsneo_discardDeviceEvents(&device.0);
    };
}

/// See [icsneo_setEventLimit()](libicsneo_sys::icsneo_setEventLimit) for more details
///
/// TODO: Description here
pub fn set_event_limit(new_limit: usize) {
    // extern void DLLExport icsneo_setEventLimit(size_t newLimit);
    unsafe {
        icsneo_setEventLimit(new_limit);
    };
}

/// See [icsneo_getEventLimit()](libicsneo_sys::icsneo_getEventLimit) for more details
///
/// TODO: Description here
pub fn get_event_limit() -> usize {
    // extern size_t DLLExport icsneo_getEventLimit(void);
    unsafe { icsneo_getEventLimit() }
}

/// See [icsneo_getSupportedDevices()](libicsneo_sys::icsneo_getSupportedDevices) for more details
///
/// TODO: Description here
pub fn get_supported_devices() -> Result<Vec<devicetype_t>> {
    // extern bool DLLExport icsneo_getSupportedDevices(devicetype_t* devices, size_t* count);
    let mut size: usize = 0;
    let success = unsafe { icsneo_getSupportedDevices(std::ptr::null_mut(), &mut size) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_getSupportedDevices() failed!".to_string(),
                ))
            }
        };
    }
    let mut device_types = Vec::with_capacity(size as usize);
    for _ in 0..size {
        device_types.push(0 as devicetype_t);
    }
    let success = unsafe { icsneo_getSupportedDevices(device_types.as_mut_ptr(), &mut size) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_getSupportedDevices() failed!".to_string(),
                ))
            }
        };
    }
    Ok(device_types)
}

/// See [icsneo_getTimestampResolution()](libicsneo_sys::icsneo_getTimestampResolution) for more details
///
/// TODO: Description here
pub fn get_timestamp_resolution(device: &NeoDevice) -> Result<u16> {
    // extern bool DLLExport icsneo_getTimestampResolution(const neodevice_t* device, uint16_t* resolution);
    let mut resolution = 0u16;
    let success = unsafe { icsneo_getTimestampResolution(&device.0, &mut resolution) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_getTimestampResolution() failed!".to_string(),
                ))
            }
        };
    }
    Ok(resolution)
}

/// See [icsneo_getDigitalIO()](libicsneo_sys::icsneo_getDigitalIO) for more details
///
/// TODO: Description here
pub fn get_digital_io(device: &NeoDevice, io_type: neoio_t, io_number: u32) -> Result<bool> {
    // extern bool DLLExport icsneo_getTimestampResolution(const neodevice_t* device, uint16_t* resolution);
    let mut value = false;
    let success = unsafe { icsneo_getDigitalIO(&device.0, io_type, io_number, &mut value) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_getDigitalIO() failed!".to_string(),
                ))
            }
        };
    }
    Ok(value)
}

/// See [icsneo_setDigitalIO()](libicsneo_sys::icsneo_setDigitalIO) for more details
///
/// TODO: Description here
pub fn set_digital_io(
    device: &NeoDevice,
    io_type: neoio_t,
    io_number: u32,
    value: bool,
) -> Result<()> {
    // extern bool DLLExport icsneo_setDigitalIO(const neodevice_t* device, neoio_t type, uint32_t number, bool value);
    let success = unsafe { icsneo_setDigitalIO(&device.0, io_type, io_number, value) };
    if !success {
        match get_last_error() {
            Some(e) => return Err(Error::ErrorOccurred(e)),
            None => {
                return Err(Error::CriticalError(
                    "icsneo_setDigitalIO() failed!".to_string(),
                ))
            }
        };
    }
    Ok(())
}

/// See [icsneo_isTerminationSupportedFor()](libicsneo_sys::icsneo_isTerminationSupportedFor) for more details
///
/// TODO: Description here
pub fn is_termination_supported_for(device: &NeoDevice, netid: neonetid_t) -> bool {
    // extern bool DLLExport icsneo_isTerminationSupportedFor(const neodevice_t* device, neonetid_t netid);
    unsafe { icsneo_isTerminationSupportedFor(&device.0, netid) }
}

/// See [icsneo_canTerminationBeEnabledFor()](libicsneo_sys::icsneo_canTerminationBeEnabledFor) for more details
///
/// TODO: Description here
pub fn can_termination_be_enabled_for(device: &NeoDevice, netid: neonetid_t) -> bool {
    // extern bool DLLExport icsneo_canTerminationBeEnabledFor(const neodevice_t* device, neonetid_t netid);
    unsafe { icsneo_canTerminationBeEnabledFor(&device.0, netid) }
}

/// See [icsneo_isTerminationEnabledFor()](libicsneo_sys::icsneo_isTerminationEnabledFor) for more details
///
/// TODO: Description here
pub fn is_termination_enabled_for(device: &NeoDevice, netid: neonetid_t) -> bool {
    // extern bool DLLExport icsneo_isTerminationEnabledFor(const neodevice_t* device, neonetid_t netid);
    unsafe { icsneo_isTerminationEnabledFor(&device.0, netid) }
}

/// See [icsneo_setTerminationFor()](libicsneo_sys::icsneo_setTerminationFor) for more details
///
/// TODO: Description here
pub fn set_termination_for(device: &NeoDevice, netid: neonetid_t, enabled: bool) -> bool {
    // extern bool DLLExport icsneo_setTerminationFor(const neodevice_t* device, neonetid_t netid, bool enabled);
    unsafe { icsneo_setTerminationFor(&device.0, netid, enabled) }
}

// TODO: extern int DLLExport icsneo_addMessageCallback(const neodevice_t* device, void (*callback)(neomessage_t), void*);
// TODO: extern bool DLLExport icsneo_removeMessageCallback(const neodevice_t* device, int id);
// TODO: extern int DLLExport icsneo_addEventCallback(void (*callback)(neoevent_t), void*);
// TODO: extern bool DLLExport icsneo_removeEventCallback(int id);
/* TODO:
        extern bool DLLExport icsneo_settingsRefresh(const neodevice_t* device);
        extern bool DLLExport icsneo_settingsApply(const neodevice_t* device);
        extern bool DLLExport icsneo_settingsApplyTemporary(const neodevice_t* device);
        extern bool DLLExport icsneo_settingsApplyDefaults(const neodevice_t* device);
        extern bool DLLExport icsneo_settingsApplyDefaultsTemporary(const neodevice_t* device);
        extern int DLLExport icsneo_settingsReadStructure(const neodevice_t* device, void* structure, size_t structureSize);
        extern bool DLLExport icsneo_settingsApplyStructure(const neodevice_t* device, const void* structure, size_t structureSize);
        extern bool DLLExport icsneo_settingsApplyStructureTemporary(const neodevice_t* device, const void* structure, size_t structureSize);
*/

#[cfg(test)]
mod tests {
    use super::*;

    enum HardwareSetup {
        ZeroDevices,
        OneDevice,
        TwoDevices,
    }

    fn get_hardware_setup() -> HardwareSetup {
        let value = match find_all_devices() {
            Ok(d) => {
                if d.len() == 0 {
                    HardwareSetup::ZeroDevices
                } else if d.len() == 1 {
                    HardwareSetup::OneDevice
                } else {
                    HardwareSetup::TwoDevices
                }
            }
            Err(e) => panic!("ERROR: {:#?}", e),
        };
        free_unconnected_devices().unwrap();
        value
    }

    #[test]
    fn test_get_hardware_setup() {
        use std::time::SystemTime;
        let start = SystemTime::now();
        for _ in 0..10000 {
            let _ = get_hardware_setup();
        }
        println!("{:?}", start.elapsed());
    }
    #[test]
    fn test_find_all_devices() {
        let _ = get_hardware_setup();
    }

    #[test]
    fn test_is_valid_neodevice() {
        // Negative test
        let invalid_device = NeoDevice::default();
        assert!(
            is_valid_neodevice(&invalid_device).is_ok(),
            "neodevice_t should have thrown an Error"
        );

        match get_hardware_setup() {
            HardwareSetup::ZeroDevices => (),
            HardwareSetup::OneDevice | HardwareSetup::TwoDevices => {
                let devices = find_all_devices().unwrap();
                // All devices here should be valid
                for device in &devices {
                    assert_eq!(is_valid_neodevice(&device).unwrap(), true);
                }
                // All devices here should be invalid
                free_unconnected_devices().unwrap();
                for device in &devices {
                    assert!(is_valid_neodevice(&device).is_ok());
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_is_open_panic() {
        let device = NeoDevice::default();
        // This will panic since we are passing in an invalid neodevice_t
        assert_eq!(is_open(&device).unwrap(), false);
    }

    #[test]
    fn test_open_online_close_device() {
        match get_hardware_setup() {
            HardwareSetup::ZeroDevices => (),
            HardwareSetup::OneDevice | HardwareSetup::TwoDevices => {
                // All devices here should be valid
                for device in find_all_devices().unwrap() {
                    // Open
                    assert_eq!(is_open(&device).unwrap(), false);
                    open_device(&device).unwrap();
                    assert_eq!(is_open(&device).unwrap(), true);
                    // Online
                    assert_eq!(is_online(&device).unwrap(), false);
                    go_online(&device).unwrap();
                    assert_eq!(is_online(&device).unwrap(), true);
                    go_offline(&device).unwrap();
                    assert_eq!(is_online(&device).unwrap(), false);

                    // Close
                    close_device(&device).unwrap();
                    // Device is no longer valid since we closed it...
                    assert!(is_valid_neodevice(&device).is_ok());
                }
                free_unconnected_devices().unwrap();
            }
        }
    }

    #[test]
    fn test_get_last_error() {
        assert!(get_last_error().is_none());
    }

    #[test]
    fn test_serial_num_to_string() {
        let result = serial_num_to_string(50000);
        assert_eq!("50000".to_string(), result.unwrap());

        let result = serial_num_to_string(783132957);
        assert_eq!("CY9999".to_string(), result.unwrap());
    }

    #[test]
    fn test_serial_string_to_num() {
        assert_eq!(50000, serial_string_to_num("50000"));
        assert_eq!(783132957, serial_string_to_num("CY9999"));
    }
}