//! Rust API bindings for Intrepid Control Systems's libicsneo C API.
//! This crate provides a safe and easy way to interact with libicsneo over
//! [libicsneo_sys](libicsneo_sys).
//! This crate is a work in progress and currently not recommended for production.
//! 
//! [GitHub libicsneo](https://github.com/intrepidcs/libicsneo)
//! 
//! [GitHub libicsneo-sys](https://github.com/intrepidcs/libicsneo-sys)
//! 
//! [GitHub libicsneo-rs](https://github.com/intrepidcs/libicsneo-rs)

pub mod native;

#[cfg(feature = "python")]
mod python;
