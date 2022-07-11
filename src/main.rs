use std::{
    io::{stdout, Write},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::Result;

use ds4linux::{curve, hid::DS4State};
use evdev_rs::{
    enums::{BusType, EventCode, EventType, EV_ABS, EV_KEY, EV_SYN},
    AbsInfo, TimeVal,
};
use evdev_rs::{DeviceWrapper, InputEvent, UInputDevice, UninitDevice};
use rusb::{Context, Device, DeviceHandle, UsbContext};

const VID: u16 = 0x054c;
const PID: u16 = 0x05c4;

const ANALOG_MAX: u8 = 255;

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

fn event_time_now() -> TimeVal {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    TimeVal::new(now.as_secs() as i64, now.subsec_nanos() as i64)
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

    // Crate uinput device (/dev/input/jn, where n is a positive integer)
    let u = UninitDevice::new().unwrap();
    u.set_name("Sony Dualshock Hackery");
    u.set_bustype(BusType::BUS_USB as u16);
    u.set_vendor_id(VID);
    u.set_product_id(PID);

    u.enable_event_type(&EventType::EV_KEY)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_WEST), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_SOUTH), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_EAST), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_NORTH), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_SELECT), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_START), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_TOUCH), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_MODE), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_TL), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_TL2), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_TR), None)?;
    u.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_TR2), None)?;

    let absinfo_dpad = AbsInfo {
        value: 0,
        minimum: -1,
        maximum: 1,
        fuzz: 0,
        flat: 0,
        resolution: 0,
    };

    let absinfo_stick = AbsInfo {
        value: 127,
        minimum: 0,
        maximum: ANALOG_MAX as i32,
        fuzz: 0,
        flat: 15,
        resolution: 0,
    };
    let absinfo_stick_r = AbsInfo {
        value: 127,
        minimum: 0,
        maximum: ANALOG_MAX as i32,
        fuzz: 0,
        flat: 5,
        resolution: 0,
    };
    u.enable_event_type(&EventType::EV_ABS)?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_HAT0X), Some(&absinfo_dpad))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_HAT0Y), Some(&absinfo_dpad))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_TILT_X), Some(&absinfo_stick))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_TILT_Y), Some(&absinfo_stick))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_RX), Some(&absinfo_stick_r))?;
    u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_RY), Some(&absinfo_stick_r))?;
    // u.enable_event_code(&EventCode::EV_ABS(EV_ABS::ABS_RY), Some(&absinfo_stick))?;

    println!("Finished setting up virtual device");

    let v = UInputDevice::create_from_device(&u)?;

    // Main loop
    let timeout = Duration::from_secs(1);

    let mut stdout = stdout();
    println!("01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 TP PS\n");
    let mut buf: [u8; 64] = [0; 64];

    let mut p_state = DS4State::initial_state();
    loop {
        handle.read_interrupt(endpoint.address, &mut buf, timeout)?;
        let event_time = event_time_now();
        let c_state = DS4State::from(&buf);

        // DEBUG OUTPUT
        // print!("\r{}", c_state);

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

        // TODO: check c_state to p_state to see if changes happened, before making c_state become p_state

        // DPAD
        if c_state.up != p_state.up || c_state.down != p_state.down {
            let mut val = 0;
            if c_state.up {
                val = -1
            } else if c_state.down {
                val = 1;
            }
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_ABS(EV_ABS::ABS_HAT0Y),
                value: val,
            })?;
        }
        if c_state.left != p_state.left || c_state.right != p_state.right {
            let mut val = 0;
            if c_state.left {
                val = -1
            } else if c_state.right {
                val = 1;
            }
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_ABS(EV_ABS::ABS_HAT0X),
                value: val,
            })?;
        }

        // Face buttons
        if c_state.square != p_state.square {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_WEST),
                value: c_state.square as i32,
            })?;
        }
        if c_state.cross != p_state.cross {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_SOUTH),
                value: c_state.cross as i32,
            })?;
        }
        if c_state.circle != p_state.circle {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_EAST),
                value: c_state.circle as i32,
            })?;
        }
        if c_state.triangle != p_state.triangle {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_NORTH),
                value: c_state.triangle as i32,
            })?;
        }

        // Triggers
        if c_state.l1 != p_state.l1 {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_TL),
                value: c_state.l1 as i32,
            })?;
        }
        if c_state.l2 != p_state.l2 {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_TL2),
                value: c_state.l2 as i32,
            })?;
        }
        if c_state.r1 != p_state.r1 {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_TR),
                value: c_state.r1 as i32,
            })?;
        }
        if c_state.r2 != p_state.r2 {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_TR2),
                value: c_state.r2 as i32,
            })?;
        }

        // PS & Touchpad
        if c_state.ps != p_state.ps {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_MODE),
                value: c_state.ps as i32,
            })?;
        }
        if c_state.touchpad != p_state.touchpad {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_TOUCH),
                value: c_state.touchpad as i32,
            })?;
        }

        // start select
        if c_state.start != p_state.start {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_START),
                value: c_state.start as i32,
            })?;
        }
        if c_state.select != p_state.select {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_KEY(EV_KEY::BTN_SELECT),
                value: c_state.select as i32,
            })?;
        }

        // Analogues
        // Left stick
        if c_state.lsx != p_state.lsx {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_ABS(EV_ABS::ABS_TILT_X),
                value: curve::custom(c_state.lsx, ANALOG_MAX) as i32,
            })?;
        }
        if c_state.lsy != p_state.lsy {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_ABS(EV_ABS::ABS_TILT_Y),
                value: curve::custom(c_state.lsy, ANALOG_MAX) as i32,
            })?;
        }
        // Right stick
        if c_state.rsx != p_state.rsx {
            // Deadzone check
            if i16::abs(c_state.rsx as i16 - 128) > 100 {
                v.write_event(&InputEvent {
                    time: event_time,
                    event_code: EventCode::EV_ABS(EV_ABS::ABS_RX),
                    value: c_state.rsx as i32,
                })?;
            } else {
                v.write_event(&InputEvent {
                    time: event_time,
                    event_code: EventCode::EV_ABS(EV_ABS::ABS_RX),
                    value: 127, // center stick value
                })?;
            }
        }
        if c_state.rsy != p_state.rsy {
            v.write_event(&InputEvent {
                time: event_time,
                event_code: EventCode::EV_ABS(EV_ABS::ABS_RY),
                value: c_state.rsy as i32,
            })?;
        }

        // Needs to be called to make written events be updated
        v.write_event(&InputEvent {
            time: event_time,
            event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
            value: 0,
        })?;

        p_state = c_state;
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
) -> rusb::Result<()> {
    handle.set_active_configuration(endpoint.config)?;
    handle.claim_interface(endpoint.iface)?;
    handle.set_alternate_setting(endpoint.iface, endpoint.setting)
}
