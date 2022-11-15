use pyo3::prelude::*;
use crate::safe::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn libicsneo(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_all_devices, m)?)?;
    m.add_function(wrap_pyfunction!(free_unconnected_devices, m)?)?;
    m.add_function(wrap_pyfunction!(is_valid_neodevice, m)?)?;
    m.add_function(wrap_pyfunction!(serial_num_to_string, m)?)?;


    m.add_function(wrap_pyfunction!(describe_device, m)?)?;
    m.add_function(wrap_pyfunction!(serial_string_to_num, m)?)?;
    m.add_function(wrap_pyfunction!(get_last_error, m)?)?;
    m.add_function(wrap_pyfunction!(open_device, m)?)?;
    m.add_function(wrap_pyfunction!(close_device, m)?)?;
    m.add_function(wrap_pyfunction!(go_online, m)?)?;
    m.add_function(wrap_pyfunction!(go_offline, m)?)?;
    m.add_function(wrap_pyfunction!(is_online, m)?)?;
    m.add_function(wrap_pyfunction!(enable_message_polling, m)?)?;
    m.add_function(wrap_pyfunction!(disable_message_polling, m)?)?;
    m.add_function(wrap_pyfunction!(is_message_polling_enabled, m)?)?;
    m.add_function(wrap_pyfunction!(get_messages, m)?)?;
    m.add_function(wrap_pyfunction!(get_polling_message_limit, m)?)?;
    m.add_function(wrap_pyfunction!(set_polling_message_limit, m)?)?;
    m.add_function(wrap_pyfunction!(get_network_by_number, m)?)?;
    m.add_function(wrap_pyfunction!(get_product_name, m)?)?;
    m.add_function(wrap_pyfunction!(get_product_name_for_type, m)?)?;
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_function(wrap_pyfunction!(get_baudrate, m)?)?;
    m.add_function(wrap_pyfunction!(set_baudrate, m)?)?;
    m.add_function(wrap_pyfunction!(get_fd_baudrate, m)?)?;
    m.add_function(wrap_pyfunction!(set_fd_baudrate, m)?)?;
    m.add_function(wrap_pyfunction!(set_write_blocks, m)?)?;
    m.add_function(wrap_pyfunction!(get_events, m)?)?;
    m.add_function(wrap_pyfunction!(get_device_events, m)?)?;
    m.add_function(wrap_pyfunction!(discard_all_events, m)?)?;
    m.add_function(wrap_pyfunction!(discard_all_device_events, m)?)?;
    m.add_function(wrap_pyfunction!(set_event_limit, m)?)?;
    m.add_function(wrap_pyfunction!(get_event_limit, m)?)?;
    m.add_function(wrap_pyfunction!(get_supported_devices, m)?)?;
    m.add_function(wrap_pyfunction!(get_timestamp_resolution, m)?)?;
    m.add_function(wrap_pyfunction!(get_digital_io, m)?)?;
    m.add_function(wrap_pyfunction!(set_digital_io, m)?)?;
    m.add_function(wrap_pyfunction!(is_termination_supported_for, m)?)?;
    m.add_function(wrap_pyfunction!(can_termination_be_enabled_for, m)?)?;
    m.add_function(wrap_pyfunction!(is_termination_enabled_for, m)?)?;
    m.add_function(wrap_pyfunction!(set_termination_for, m)?)?;
    m.add_function(wrap_pyfunction!(transmit, m)?)?;
    //m.add_function(wrap_pyfunction!(transmit_messages, m)?)?;
    
    
    

    m.add_class::<NeoDevice>()?;
    //m.add_class::<Error>();
    Ok(())
}