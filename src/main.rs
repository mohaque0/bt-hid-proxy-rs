mod keys;

use clap::Parser;
use evdev::{Device, Synchronization, Key};
use std::{io::{self, Write}, fs::File};

const KEY_PRESSED_VALUE: i32 = 1;
const KEY_NOT_PRESSED_VALUE: i32 = 0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// If true logs all events to program output. SECURITY RISK.
    #[arg(short, long, default_value_t = false)]
    log_events: bool,

    /// Virtual file to open to get keyboard input.
    #[arg(short, long, default_value_t = String::from("/dev/input/event0"))]
    dev_input_event_file: String
}

fn main() {
    let args = Args::parse();

    match try_proxy_hid_events(args) {
        Ok(_) => println!("Exiting normally."),
        Err(e) => println!("Error: {}", e),
    }
}

fn try_proxy_hid_events(args: Args) -> io::Result<()> {
    println!("Starting hid proxy.");
    println!("Open {}", args.dev_input_event_file);

    let mut device = Device::open(args.dev_input_event_file)?;

    println!("Grab device.");
    device.grab()?;

    println!("Done");

    let mut modifiers = 0;
    let mut keys = vec![];

    loop {

        for event in device.fetch_events()? {
            if args.log_events {
                println!("{:?}", event);
            }

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