#[macro_use]
extern crate lazy_static;
use std::time::Duration;

use rusb::{
    request_type, Context, Device, DeviceHandle, Direction, RequestType, Result, UsbContext,
};

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
    // control device here

    // set_idle(&mut handle)?;
    // let descriptor = get_descriptor(&mut handle)?;
    // println!("Descriptor: {}", &descriptor);

    // Main loop
    let timeout = Duration::from_secs(1);

    loop {
        let mut buf: Vec<u8> = vec![0; 64];
        handle.read_interrupt(endpoint.address, &mut buf, timeout)?;
        println!("{:?}", buf);
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

fn set_idle<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<usize> {
    lazy_static! {
        static ref REQUEST_TYPE: u8 = request_type(
            Direction::Out,
            rusb::RequestType::Class,
            rusb::Recipient::Interface,
        );
    }
    let timeout = Duration::from_secs(1);
    // Const values are picked directly from the package capture data
    const REQUEST: u8 = 0x0A;
    const VALUE: u16 = 0x0000;
    const INDEX: u16 = 0x0000;
    handle.write_control(*REQUEST_TYPE, REQUEST, VALUE, INDEX, &[], timeout)
}

fn response_set_idle<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<Vec<u8>> {
    let timeout = Duration::from_secs(1);
    // Const values are picked directly from the package capture data
    const REQUEST_TYPE: u8 = 0x0;
    const REQUEST: u8 = 0x0;
    const VALUE: u16 = 0x0000;
    const INDEX: u16 = 0x0000;
    let mut buffer: Vec<u8> = vec![];
    handle.read_control(0x80, REQUEST, VALUE, INDEX, &mut buffer, timeout)?;
    println!("Response: {:?}", buffer);
    Ok(buffer)
}

fn get_descriptor<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<String> {
    lazy_static! {
        static ref REQUEST_TYPE: u8 = request_type(
            Direction::Out,
            rusb::RequestType::Class,
            rusb::Recipient::Interface,
        );
    }

    let timeout = Duration::from_secs(1);
    // Const values are picked directly from the package capture data
    let languages = handle.read_languages(timeout)?;
    println!("{:?}", languages);
    handle.read_string_descriptor(languages[0], 0x0, timeout)
}

// fn get_report<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<usize> {
//     let timeout = Duration::from_secs(1);

//     // values are picked directly from the captured packet
//     const REQUEST_TYPE: u8 = 0xa1;
//     const REQUEST: u8 = 0x01;
//     const VALUE: u16 = 0x0200;
//     const INDEX: u16 = 0x0000;
//     const DATA: [u8; 64] = [
//         0x3f, 0x10, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x5b,
//     ];

//     let mut buffer: Vec<u8> = vec![];

//     let res = handle.read_control(REQUEST_TYPE, REQUEST, VALUE, INDEX, buffer, timeout)?;
// }