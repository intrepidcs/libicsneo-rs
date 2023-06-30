use libicsneo::icsneo::{find_all_devices, free_unconnected_devices, get_version, describe_device};
fn main() {
    println!("libicsneo version: {}", get_version().version_as_str().unwrap());

    println!("Finding devices...");
    let devices = find_all_devices().unwrap();
    println!("Found {} device(s)", devices.len());
    for (i, device) in devices.iter().enumerate() {
        let description = match describe_device(device) {
            Ok(d) => d,
            Err(_) => device.serial(),
        };
        println!("\t{}. {}", i+1, description);
    }
    free_unconnected_devices().unwrap();
}
