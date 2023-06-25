//! Rust sys level crate providing a cbindgen API for libicsneo (https://github.com/intrepidcs/libicsneo)
//! If unsure, don't use this crate and use libicsneo-rs instead.
// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_icsneo_findAllDevices() {
        const DEVICE_COUNT_MAX: usize = 255;
        let mut device_count = DEVICE_COUNT_MAX;
        let mut devices = [neodevice_t::default(); DEVICE_COUNT_MAX];
        
        unsafe {
            icsneo_findAllDevices(devices.as_mut_ptr(), &mut device_count);
        }
        println!("Found {} device(s)", device_count);

        for i in 0..device_count as usize {
            let device = &devices[i];
            let success;
            let name = unsafe {
                // Open the device
                success = icsneo_openDevice(device);

                // Get the device type string
                let mut name_length: usize = 0;
                icsneo_getProductNameForType(device.type_, std::ptr::null_mut(), &mut name_length);
                name_length += 1;
                let mut name_buffer = vec![0 as std::os::raw::c_char; name_length as usize];
                icsneo_getProductNameForType(
                    device.type_,
                    name_buffer.as_mut_ptr(),
                    &mut name_length,
                );
                let name =
                    CString::from_vec_with_nul(name_buffer.iter().map(|v| *v as u8).collect())
                        .unwrap();
                let name = name.to_str().unwrap();
                format!("{} {}", name, "TODO") //device.get_serial_number())
            };
            if success {
                println!("Opened {} successfully:\n{:#?}", name, device);
                unsafe {
                    let _ = icsneo_closeDevice(device);
                }
            } else {
                println!("Failed to open {:?}!", device);
            }
        }
    }
}
