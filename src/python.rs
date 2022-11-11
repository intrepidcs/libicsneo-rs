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

    m.add_class::<NeoDevice>()?;
    //m.add_class::<Error>();
    Ok(())
}