use std::{
    io::{stdout, Write},
    time::Duration,
};

use ds4linux::hid::DS4State;
use rusb::{Context, Device, DeviceHandle, Result, UsbContext};

const VID: u16 = 0x054c;
const PID: u16 = 0x05c4;

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

fn main() -> Result<()> {
    let mut context = Context::new()?;
    let (mut device, mut handle) = open_device(&mut context, VID, PID).expect("Did not find USB device (if connected, perhaps you're not allowed to read from the device?)");

    print_device_info(&mut handle)?;

    let endpoints = find_readable_endpoints(&mut device)?;
    let endpoint = endpoints
        .first()
        .expect("No configurable endpoint found on device");

    println!("endpoints: {:?}", endpoints);

    let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
        Ok(true) => {
            handle.detach_kernel_driver(endpoint.iface)?;
            true
        }
        _ => false,
    };

    println!("Has kernel driver? {}", has_kernel_driver);

    // clam and configure device
    configure_endpoint(&mut handle, &endpoint)?;

    // Main loop
    let timeout = Duration::from_secs(1);

    let mut stdout = stdout();
    println!("01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 TP PS\n");
    let mut buf: [u8; 64] = [0; 64];
    loop {
        handle.read_interrupt(endpoint.address, &mut buf, timeout)?;
        let c_state = DS4State::from(&buf);

        // DEBUG OUTPUT
        print!("\r{}", c_state);

        // RAW DEBUG OUTPUT
        // let outstr = buf
        //     .iter()
        //     .map(|b| format!("{:02X}", b))
        //     .collect::<Vec<String>>()
        //     .join(" ");
        // let touchpad_down = buf[7] & 0x02;
        // let ps_button = buf[7] & 0x1;
        // print!("\r{}", outstr);
        // print!(" {:02X}", touchpad_down);
        // print!(" {:02X}", ps_button);
        stdout.flush().unwrap();
    }

    // cleanup after use
    handle.release_interface(endpoint.iface)?;
    if has_kernel_driver {
        handle.attach_kernel_driver(endpoint.iface)?;
    }
    Ok(())
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, handle)),
                Err(e) => println!("Failure to read ds4: {:?}", e),
            }
        }
    }

    None
}

fn print_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<()> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!("Active configuration: {}", handle.active_configuration()?);

    if !languages.is_empty() {
        let language = languages[0];
        println!("Language: {:?}", language);

        println!(
            "Manufacturer: {}",
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Product: {}",
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Serial Number: {}",
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
    }
    Ok(())
}

// returns all readable endpoints for given usb device and descriptor
fn find_readable_endpoints<T: UsbContext>(device: &mut Device<T>) -> Result<Vec<Endpoint>> {
    let device_desc = device.device_descriptor()?;
    let mut endpoints = vec![];
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // println!("{:#?}", config_desc);
        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                // println!("{:#?}", interface_desc);
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    // println!("{:#?}", endpoint_desc);
                    endpoints.push(Endpoint {
                        config: config_desc.number(),
                        iface: interface_desc.interface_number(),
                        setting: interface_desc.setting_number(),
                        address: endpoint_desc.address(),
                    });
                }
            }
        }
    }

    Ok(endpoints)
}

fn configure_endpoint<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    endpoint: &Endpoint,
) -> Result<()> {
    handle.set_active_configuration(endpoint.config)?;
    handle.claim_interface(endpoint.iface)?;
    handle.set_alternate_setting(endpoint.iface, endpoint.setting)
}
