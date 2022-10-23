use libicsneo_sys::*;

trait SerialNumber {
    fn get_serial_number(&self) -> String;
}

impl SerialNumber for neodevice_t {
    fn get_serial_number(&self) -> String {
        let serial: String = self.serial.map(|v| v as u8 as char).into_iter().collect();
        serial
    }
}


pub mod icsneo {
    use libicsneo_sys::{neodevice_t, icsneo_findAllDevices};

    #[derive(Debug)]
    pub enum Error {
        NoDevicesFound,
    }

    /// TODO
    pub fn find_all_devices() -> Result<Vec<neodevice_t>, Error> {
        // Get the device count
        let device_count = unsafe {
            let mut device_count = 0;
            icsneo_findAllDevices(std::ptr::null_mut(), &mut device_count);
            // TODO: Check error codes here
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
            // TODO: Check error codes here
        }
        Ok(devices)
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
}
