//! Rust API for libicsneo.
//! TODO: Add a better description here


pub mod icsneo {
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
        NoDevicesFound,
        ErrorOccurred(neoevent_t),
    }

    /// Calls icsneo_findAllDevices().
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

    /// Calls icsneo_getLastError(). Returns the neoevent_t if an error occurred or None if not.
    pub fn get_last_error() -> Option<neoevent_t> {
        // TODO: Does description need to be allocated here?
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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_all_devices() {
        // TODO
        let results = icsneo::find_all_devices();
        // Will only be true if no devices are plugged in
        assert_eq!(results.is_err(), true);
    }

    #[test]
    fn test_get_last_error() {
        //assert_matches!(icsneo::get_last_error(), None);
    }
}
