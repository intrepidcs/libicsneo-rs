//! Rust API for libicsneo.
//! TODO: Add a better description here


pub mod icsneo {
    use std::ffi::{CStr, CString};

    use libicsneo_sys::*;

    trait SerialNumber {
        fn get_serial_number(&self) -> String;
    }
    
    impl SerialNumber for libicsneo_sys::neodevice_t {
        fn get_serial_number(&self) -> String {
            let serial: String = self.serial.map(|v| v as u8 as char).into_iter().collect();
            serial
        }
    }

    #[derive(Debug)]
    pub enum Error {
        // No devices were found.
        NoDevicesFound,
        // icsneo_getLastError() happened.
        ErrorOccurred(neoevent_t),
        // Critical API error that shouldn't have happened.
        CriticalError(String),
    }

    /// Find all Intrepid devices connected. Returns a Result of Vec<neodevice_t>. See icsneo_findAllDevices for more details.
    /// 
    /// Example:
    /// ```
    /// /*
    /// use libicsneo_rs::*;
    /// 
    /// let devices = icsneo::find_all_devices();
    /// if devices.is_ok() {
    ///     let devices = devices.unwrap();
    ///     for device in devices {
    ///         let serial: String = device.serial.map(|v| v as u8 as char).into_iter().collect();
    ///         println!("Found device: {serial}");
    ///     }
    /// }
    /// */
    /// ```
    pub fn find_all_devices() -> Result<Vec<neodevice_t>, Error> {
        // Get the device count
        let device_count = unsafe {
            let mut device_count = 0;
            icsneo_findAllDevices(std::ptr::null_mut(), &mut device_count);
            match get_last_error() {
                Some(e) => return Err(Error::ErrorOccurred(e)),
                None => {},
            };
            device_count
        };
        // We are done if we don't have any devices
        if device_count == 0 {
            return Err(Error::NoDevicesFound);
        }
        let mut devices = Vec::with_capacity(device_count as usize);
        for _ in 0..device_count {
            devices.push(neodevice_t {
                device: 0 as *mut std::os::raw::c_void,
                handle: 0i32,
                serial: [0i8; 7],
                type_: 0,
            });
        }
        unsafe {
            let mut device_count = device_count;
            icsneo_findAllDevices(devices.as_mut_ptr(), &mut device_count);
            // We are done if we don't have any devices - this should never happen 
            if device_count == 0 {
                return Err(Error::NoDevicesFound);
            }
            match get_last_error() {
                Some(e) => return Err(Error::ErrorOccurred(e)),
                None => {},
            };
        }
        Ok(devices)
    }

    /// Frees all unconnected devices. See icsneo_freeUnconnectedDevices for more details.
    pub fn free_unconnected_devices() {
        // extern void DLLExport icsneo_freeUnconnectedDevices();
        unsafe {
            icsneo_freeUnconnectedDevices();
        }
    }

    /// Converts a serial number integer to a string. See icsneo_serialNumToString for more details.
    pub fn serial_num_to_string(num: u32) -> Result<String, Error> {
        // extern bool DLLExport icsneo_serialNumToString(uint32_t num, char* str, size_t* count);
        
        // Grab the length needed
        let mut count = 0u64;
        let success = unsafe {
            icsneo_serialNumToString(num, std::ptr::null_mut(), &mut count)
        };
        // icsneo_serialNumToString returns false when we query for the str length.
        if success {
            return Err(Error::CriticalError("icsneo_serialNumToString() failed to query length!".to_string()));
        }
        // Need to account for the null terminator to prevent OBOE
        count += 1;
        let mut buffer: Vec<i8> = vec![0; count as usize];
        let success = unsafe { 
            icsneo_serialNumToString(num, buffer.as_mut_ptr(), &mut count)
        };
        if !success {
            let _result = get_last_error();
            return Err(Error::CriticalError("icsneo_serialNumToString() failed to convert!".to_string()));
        }
        // Convert the CStr to a String on success
        unsafe {
            return match CStr::from_ptr(buffer.as_mut_ptr()).to_owned().to_str() {
                Ok(s) => Ok(s.to_string()),
                Err(e) => {
                    let msg = format!("Failed to convert serial number buffer to CString: {e}");
                    Err(Error::CriticalError(msg))
                },
            };
        };
    }

    /// Converts a serial number string to a base10 integer. See icsneo_serialStringToNum for more details.
    pub fn serial_string_to_num(serial_str: &str) -> u32 {
        // extern uint32_t DLLExport icsneo_serialStringToNum(const char* str);
        let serial = CString::new(serial_str).unwrap();
        unsafe {
            icsneo_serialStringToNum(serial.as_ptr())
        }
    }

    /// Returns the neoevent_t if an error occurred or None if none. See icsneo_getLastError() for more details.
    pub fn get_last_error() -> Option<neoevent_t> {
        let mut neo_event = neoevent_t {
            description: 0 as *const std::os::raw::c_char,
            timestamp: 0,
            eventNumber: 0,
            severity: 0,
            serial: [0i8; 7],
            reserved: [0u8; 16],
        };
        unsafe {
            if icsneo_getLastError(&mut neo_event as *mut _) {
                Some(neo_event)
            } else {
                None
            }
        }
    }

    /// See icsneo_isValidNeoDevice for more details.
    pub fn is_valid_neodevice(device: &neodevice_t) -> bool {
        // extern bool DLLExport icsneo_isValidNeoDevice(const neodevice_t* device);
        unsafe {
            icsneo_isValidNeoDevice(device)
        }
    }

    /// Opens a neo device. See icsneo_openDevice for more details.
    pub fn open_device(device: &neodevice_t) -> Result<bool, Error> {
        // extern bool DLLExport icsneo_openDevice(const neodevice_t* device);
        let success = unsafe {
            icsneo_openDevice(device)
        };
        if !success {
            match get_last_error() {
                Some(e) => return Err(Error::ErrorOccurred(e)),
                None => {},
            };
        }
        Ok(success)
    }

    /// Closes a neo device. See icsneo_closeDevice for more details.
    pub fn close_device(device: &neodevice_t) -> Result<bool, Error> {
        // extern bool DLLExport icsneo_closeDevice(const neodevice_t* device);
        let success = unsafe {
            icsneo_closeDevice(device)
        };
        if !success {
            match get_last_error() {
                Some(e) => return Err(Error::ErrorOccurred(e)),
                None => {},
            };
        }
        Ok(success)
    }

    /// Checks to see if a neo device is open. See icsneo_isOpen for more details.
    pub fn is_open(device: &neodevice_t) -> Result<bool, Error> {
        // extern bool DLLExport icsneo_isOpen(const neodevice_t* device);
        let success = unsafe {
            icsneo_isOpen(device)
        };
        // TODO: Is this needed?
        /*
        if !success {
            match get_last_error() {
                Some(e) => return Err(Error::ErrorOccurred(e)),
                None => {},
            };
        }
        */
        Ok(success)
    }

    /// Goes online with a neo device. See icsneo_goOnline for more details.
    pub fn go_online(device: &neodevice_t) -> Result<bool, Error> {
        // extern bool DLLExport icsneo_goOnline(const neodevice_t* device);
        let success = unsafe {
            icsneo_goOnline(device)
        };
        if !success {
            match get_last_error() {
                Some(e) => return Err(Error::ErrorOccurred(e)),
                None => {},
            };
        }
        Ok(success)
    }

    /// Goes online with a neo device. See icsneo_goOnline for more details.
    pub fn go_offline(device: &neodevice_t) -> Result<bool, Error> {
        // extern bool DLLExport icsneo_goOffline(const neodevice_t* device);
        let success = unsafe {
            icsneo_goOffline(device)
        };
        if !success {
            match get_last_error() {
                Some(e) => return Err(Error::ErrorOccurred(e)),
                None => {},
            };
        }
        Ok(success)
    }

    /// Checks if the neo device is online. See icsneo_isOnline for more details.
    pub fn is_online(device: &neodevice_t) -> Result<bool, Error> {
        // extern bool DLLExport icsneo_isOnline(const neodevice_t* device);
        let success = unsafe {
            icsneo_isOnline(device)
        };
        if !success {
            match get_last_error() {
                Some(e) => return Err(Error::ErrorOccurred(e)),
                None => {},
            };
        }
        Ok(success)
    }

    /*
    extern void DLLExport icsneo_findAllDevices(neodevice_t* devices, size_t* count);
    extern void DLLExport icsneo_freeUnconnectedDevices();
    extern bool DLLExport icsneo_serialNumToString(uint32_t num, char* str, size_t* count);
    extern uint32_t DLLExport icsneo_serialStringToNum(const char* str);
    extern bool DLLExport icsneo_isValidNeoDevice(const neodevice_t* device);
    extern bool DLLExport icsneo_openDevice(const neodevice_t* device);
    extern bool DLLExport icsneo_closeDevice(const neodevice_t* device);
    extern bool DLLExport icsneo_isOpen(const neodevice_t* device);
    extern bool DLLExport icsneo_goOnline(const neodevice_t* device);
    extern bool DLLExport icsneo_goOffline(const neodevice_t* device);
    extern bool DLLExport icsneo_isOnline(const neodevice_t* device);
    // TODO: PLACEHOLDER: Next ones on the list:
    extern bool DLLExport icsneo_enableMessagePolling(const neodevice_t* device);
    extern bool DLLExport icsneo_disableMessagePolling(const neodevice_t* device);
    extern bool DLLExport icsneo_isMessagePollingEnabled(const neodevice_t* device);
    extern bool DLLExport icsneo_getMessages(const neodevice_t* device, neomessage_t* messages, size_t* items, uint64_t timeout);
    extern int DLLExport icsneo_getPollingMessageLimit(const neodevice_t* device);
    extern bool DLLExport icsneo_setPollingMessageLimit(const neodevice_t* device, size_t newLimit);
    extern int DLLExport icsneo_addMessageCallback(const neodevice_t* device, void (*callback)(neomessage_t), void*);
    extern bool DLLExport icsneo_removeMessageCallback(const neodevice_t* device, int id);
    extern neonetid_t DLLExport icsneo_getNetworkByNumber(const neodevice_t* device, neonettype_t type, unsigned int number);
    extern bool DLLExport icsneo_getProductName(const neodevice_t* device, char* str, size_t* maxLength);
    extern bool DLLExport icsneo_getProductNameForType(devicetype_t type, char* str, size_t* maxLength);
    extern bool DLLExport icsneo_settingsRefresh(const neodevice_t* device);
    extern bool DLLExport icsneo_settingsApply(const neodevice_t* device);
    extern bool DLLExport icsneo_settingsApplyTemporary(const neodevice_t* device);
    extern bool DLLExport icsneo_settingsApplyDefaults(const neodevice_t* device);
    extern bool DLLExport icsneo_settingsApplyDefaultsTemporary(const neodevice_t* device);
    extern int DLLExport icsneo_settingsReadStructure(const neodevice_t* device, void* structure, size_t structureSize);
    extern bool DLLExport icsneo_settingsApplyStructure(const neodevice_t* device, const void* structure, size_t structureSize);
    extern bool DLLExport icsneo_settingsApplyStructureTemporary(const neodevice_t* device, const void* structure, size_t structureSize);
    extern int64_t DLLExport icsneo_getBaudrate(const neodevice_t* device, neonetid_t netid);
    extern bool DLLExport icsneo_setBaudrate(const neodevice_t* device, neonetid_t netid, int64_t newBaudrate);
    extern int64_t DLLExport icsneo_getFDBaudrate(const neodevice_t* device, neonetid_t netid);
    extern bool DLLExport icsneo_setFDBaudrate(const neodevice_t* device, neonetid_t netid, int64_t newBaudrate);
    extern bool DLLExport icsneo_transmit(const neodevice_t* device, const neomessage_t* message);
    extern bool DLLExport icsneo_transmitMessages(const neodevice_t* device, const neomessage_t* messages, size_t count);
    extern void DLLExport icsneo_setWriteBlocks(const neodevice_t* device, bool blocks);
    extern bool DLLExport icsneo_describeDevice(const neodevice_t* device, char* str, size_t* maxLength);
    extern neoversion_t DLLExport icsneo_getVersion(void);
    extern int DLLExport icsneo_addEventCallback(void (*callback)(neoevent_t), void*);
    extern bool DLLExport icsneo_removeEventCallback(int id);
    extern bool DLLExport icsneo_getEvents(neoevent_t* events, size_t* size);
    extern bool DLLExport icsneo_getDeviceEvents(const neodevice_t* device, neoevent_t* events, size_t* size);
    extern bool DLLExport icsneo_getLastError(neoevent_t* error);
    extern void DLLExport icsneo_discardAllEvents(void);
    extern void DLLExport icsneo_discardDeviceEvents(const neodevice_t* device);
    extern void DLLExport icsneo_setEventLimit(size_t newLimit);
    extern size_t DLLExport icsneo_getEventLimit(void);
    extern bool DLLExport icsneo_getSupportedDevices(devicetype_t* devices, size_t* count);
    extern bool DLLExport icsneo_getTimestampResolution(const neodevice_t* device, uint16_t* resolution);
    extern bool DLLExport icsneo_getDigitalIO(const neodevice_t* device, neoio_t type, uint32_t number, bool* value);
    extern bool DLLExport icsneo_setDigitalIO(const neodevice_t* device, neoio_t type, uint32_t number, bool value);
    extern bool DLLExport icsneo_isTerminationSupportedFor(const neodevice_t* device, neonetid_t netid);
    extern bool DLLExport icsneo_canTerminationBeEnabledFor(const neodevice_t* device, neonetid_t netid);
    extern bool DLLExport icsneo_isTerminationEnabledFor(const neodevice_t* device, neonetid_t netid);
    extern bool DLLExport icsneo_setTerminationFor(const neodevice_t* device, neonetid_t netid, bool enabled);

    */
}

#[cfg(test)]
mod tests {

    use super::*;

    #[cfg(feature = "test_zero_devices")]
    fn cfg_test_find_all_devices() {
        match icsneo::find_all_devices() {
            Err(e) => {
                match e {
                    icsneo::Error::NoDevicesFound => assert!(true),
                    _ => assert!(false),
                };
            },
            _ => assert!(false),
        };
    }

    #[cfg(not(feature = "test_zero_devices"))]
    fn cfg_test_find_all_devices() {
        match icsneo::find_all_devices() {
            Ok(_) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_find_all_devices() {
        cfg_test_find_all_devices();
    }

    #[test]
    fn test_get_last_error() {
        assert!(icsneo::get_last_error().is_none());
    }

    #[test]
    fn test_free_unconnected_devices() {
        icsneo::free_unconnected_devices();
    }

    #[test]
    fn test_serial_num_to_string() {
        let result = icsneo::serial_num_to_string(50000);
        assert_eq!("50000".to_string(), result.unwrap());

        let result = icsneo::serial_num_to_string(783132957);
        assert_eq!("CY9999".to_string(), result.unwrap());
    }

    #[test]
    fn test_serial_string_to_num() {
        assert_eq!(50000, icsneo::serial_string_to_num("50000"));
        assert_eq!(783132957, icsneo::serial_string_to_num("CY9999"));
    }

    #[test]
    fn test_is_valid_neodevice() {
        use libicsneo_sys::neodevice_t;

        let device = neodevice_t {
            device: 0 as *mut std::os::raw::c_void,
            handle: 0i32,
            serial: [0i8; 7],
            type_: 0,
        };

        assert_eq!(icsneo::is_valid_neodevice(&device), false);

        // TODO: Need a positive test here.
    }

    #[test]
    fn test_open_device() {
        // TODO
    }

    #[test]
    fn test_close_device() {
        // TODO
    }

    #[test]
    fn test_is_open() {
        use libicsneo_sys::neodevice_t;

        let device = neodevice_t {
            device: 0 as *mut std::os::raw::c_void,
            handle: 0i32,
            serial: [0i8; 7],
            type_: 0,
        };

        assert_eq!(icsneo::is_open(&device).unwrap(), false);

        // TODO: Need a positive test here.
    }

    #[test]
    fn test_online() {
        use libicsneo_sys::neodevice_t;

        let _device = neodevice_t {
            device: 0 as *mut std::os::raw::c_void,
            handle: 0i32,
            serial: [0i8; 7],
            type_: 0,
        };

        // TODO:
        /*
        assert_eq!(icsneo::go_online(&device).unwrap(), false);
        assert_eq!(icsneo::is_online(&device).unwrap(), true);
        assert_eq!(icsneo::go_offline(&device).unwrap(), false);
        */

        // TODO: Need a positive test here.
    }

    
}
