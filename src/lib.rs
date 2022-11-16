//! Rust API for libicsneo.
//! TODO: Add a better description here

use libicsneo_sys::neodevice_t;

pub mod safe;

#[cfg(feature = "python")]
mod python;

/// Represents an Intrepid Control Systems Device
#[derive(Debug)]
pub struct NeoDevice {
    pub neodevice: neodevice_t,
}

pub enum Error {
    NeoError(safe::Error),
}

impl From<safe::Error> for Error {
    fn from(error: safe::Error) -> Self {
        Error::NeoError(error)
    }
}

impl NeoDevice {
    /// Constructs a new NeoDevice with a zero initialized neodevice
    pub fn new() -> Self {
        Self {
            neodevice: neodevice_t {
                device: 0 as *mut std::os::raw::c_void,
                handle: 0i32,
                serial: [0i8; 7],
                type_: 0,
            },
        }
    }

    /// create a [NeoDevice] from a [neodevice_t](libicsneo_sys::neodevice_t)
    pub fn from(neodevice: neodevice_t) -> Self {
        Self {
            neodevice,
        }
    }

    // TODO
    /*
    pub fn find_all() -> Result<Vec<Self>, Error> {
        let devices = match safe::find_all_devices() {
            Ok(d) => {
                d.iter().map(|d| {
                    Self::from(*d)
                }).collect()
            },
            Err(e) => return Err(Error::NeoError(e)),
        };
        Ok(devices)
    }
    */

    // TODO
    /*
    pub fn open(&self) -> Result<(), Error> {
        safe::open_device(&self.neodevice)?;
    }
    */

}