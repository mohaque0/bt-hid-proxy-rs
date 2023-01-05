mod keys;

use std::{io::{self, Write}, fs::File};
use evdev::{Device, Synchronization, Key};

const KEY_PRESSED_VALUE: i32 = 1;
const KEY_NOT_PRESSED_VALUE: i32 = 0;

fn main() {
    match try_main() {
        Ok(_) => println!("Exiting normally."),
        Err(e) => println!("Error: {}", e),
    }
}

fn try_main() -> io::Result<()> {
    println!("Starting hid proxy.");
    println!("Open /dev/input/event0.");

    let mut device = Device::open("/dev/input/event0")?;

    println!("Grab device.");
    device.grab()?;

    println!("Done");

    let mut modifiers = 0;
    let mut keys = vec![];

    loop {

        for event in device.fetch_events()? {
            match event.kind() {
                evdev::InputEventKind::Synchronization(sync) => {
                    if sync == Synchronization::SYN_DROPPED {
                        println!("Dropped events.");
                        modifiers = 0;
                        keys = vec![]
                    }
                },
                evdev::InputEventKind::Key(key) => {
                    let modifier = keys::modifiers(key);
                    if modifier != 0 {
                        if event.value() == KEY_PRESSED_VALUE {
                            modifiers |= modifier;
                        } else if event.value() == KEY_NOT_PRESSED_VALUE {
                            modifiers &= !modifier;
                        }
                    }
                    println!("Modifiers: {}", modifiers);

                    let hid_symbol = keys::scan_to_hid(&key);
                    if hid_symbol != 0 {
                        if event.value() == KEY_PRESSED_VALUE {
                            keys.push(key)
                        } else if event.value() == KEY_NOT_PRESSED_VALUE {
                            keys.retain(|it| it != &key);
                        }
                    }
                },
                evdev::InputEventKind::RelAxis(_) => {},
                evdev::InputEventKind::AbsAxis(_) => {},
                evdev::InputEventKind::Misc(_) => {},
                evdev::InputEventKind::Switch(_) => {},
                evdev::InputEventKind::Led(_) => {},
                evdev::InputEventKind::Sound(_) => {},
                evdev::InputEventKind::ForceFeedback(_) => {},
                evdev::InputEventKind::ForceFeedbackStatus(_) => {},
                evdev::InputEventKind::UInput(_) => {},
                evdev::InputEventKind::Other => {},
            }
        }

        write_report(modifiers, &keys)?;
    }
}

fn write_report(modifiers: u8, keys: &Vec<Key>) -> io::Result<()> {
    let mut report = [0u8; 8];
    report[0] = modifiers;
    // skip report[1] because report desc sends LED data here.
    report[2] = keys.get(0).map(keys::scan_to_hid).unwrap_or(0u8);
    report[3] = keys.get(1).map(keys::scan_to_hid).unwrap_or(0u8);
    report[4] = keys.get(2).map(keys::scan_to_hid).unwrap_or(0u8);
    report[5] = keys.get(3).map(keys::scan_to_hid).unwrap_or(0u8);
    report[6] = keys.get(4).map(keys::scan_to_hid).unwrap_or(0u8);
    report[7] = keys.get(5).map(keys::scan_to_hid).unwrap_or(0u8);

    let mut dst = File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .open("/dev/hidg0")?;
    dst.write_all(&report)
}