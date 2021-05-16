use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, MouseButton, MouseControllable};

use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{env, process, thread};

const LEFT_MOUSE_POS: usize = 1;
const RIGHT_MOUSE_POS: usize = 2;
const MIDDLE_MOUSE_POS: usize = 3;

static THE_SWITCH: AtomicBool = AtomicBool::new(true);
static USE_LEFT_MOUSE: AtomicBool = AtomicBool::new(true);
static USE_RIGHT_MOUSE: AtomicBool = AtomicBool::new(false);
static USE_MIDDLE_MOUSE: AtomicBool = AtomicBool::new(false);

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() == 2 {
        handle_two_arguments(&arguments);
    } else if arguments.len() == 3 {
        // We need 2 or 3 arguments
        handle_three_arguments(&arguments);
    } else {
        println!(
            "\nNo arguments found.\n
        Enter a number in milliseconds\n
        If two numbers are specified, a random number between the two values is used\n\n
        END key quits the program\n
        PAGEDOWN key turns auto click on/off\n\n
        Default the LEFT mouse button auto click is ON\n
        Toggle on/off with Numpad-1\n\n
        Default the RIGHT mouse button auto click is OFF\n
        Toggle on/off with Numpad-2\n\n
        Default the MIDDLE mouse button auto click is OFF\n
        Toggle on/off with Numpad-3\n\n"
        );
    }
}

fn handle_two_arguments(arguments: &Vec<String>) {
    let mut mouse_thread_running = false;

    let millisec = arguments[1].parse::<u64>();
    // Only if it's a number
    if millisec.is_ok() {
        let millisec_mouse = millisec.unwrap();

        // Check if we don't go below 50 milliseconds
        if millisec_mouse >= 50 {
            mouse_thread_running = true;

            thread::spawn(move || {
                squeak_the_mouse(millisec_mouse, millisec_mouse);
            });
        } else {
            println!("\nNumber must be greater than or equal to 50");
        }
    } else {
        println!("\nArgument can only be a number");
    }

    // Only check keys if mouse thread is also running
    if mouse_thread_running {
        // Always check without sleep time
        squeak_the_keys();
    }
}
fn handle_three_arguments(arguments: &Vec<String>) {
    let mut mouse_thread_running = false;

    let millisec_one = arguments[1].parse::<u64>();
    let millisec_two = arguments[2].parse::<u64>();

    if millisec_one.is_ok() && millisec_two.is_ok() {
        let millisec_mouse_one = millisec_one.unwrap();
        let millisec_mouse_two = millisec_two.unwrap();
        // Check if we don't go below 50 milliseconds
        if millisec_mouse_one >= 50 {
            if millisec_mouse_two > millisec_mouse_one {
                mouse_thread_running = true;

                thread::spawn(move || {
                    squeak_the_mouse(millisec_mouse_one, millisec_mouse_two);
                });
            } else {
                println!("\nFirst number must be lower then second number");
            }
        } else {
            println!("\nFirst number must be greater than or equal to 50");
        }
    } else {
        println!("\nArguments can only be numbers");
    }

    // Only check keys if mouse thread is also running
    if mouse_thread_running {
        // Always check without sleep time
        squeak_the_keys();
    }
}

fn squeak_the_mouse(millisec_one: u64, millisec_two: u64) {
    let device_state = DeviceState::new();
    let mut enigo = Enigo::new();

    loop {
        let mouse = device_state.get_mouse();

        // Only proceed if the auto click is enabled
        if THE_SWITCH.load(Ordering::Relaxed) {
            // If mouse button is being pressed then send a mouse event
            // creating a auto click
            if USE_LEFT_MOUSE.load(Ordering::Relaxed) && mouse.button_pressed[LEFT_MOUSE_POS] {
                enigo.mouse_down(MouseButton::Left);
            } else if USE_RIGHT_MOUSE.load(Ordering::Relaxed)
                && mouse.button_pressed[RIGHT_MOUSE_POS]
            {
                enigo.mouse_down(MouseButton::Right);
            } else if USE_MIDDLE_MOUSE.load(Ordering::Relaxed)
                && mouse.button_pressed[MIDDLE_MOUSE_POS]
            {
                enigo.mouse_down(MouseButton::Middle);
            }
        }

        // Sleep for a random time
        let sleep_time = rand::thread_rng().gen_range(millisec_one..=millisec_two);
        thread::sleep(Duration::from_millis(sleep_time));
    }
}

fn squeak_the_keys() {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];

    loop {
        // Reverse the keys. Because second key gets position 0 and the first postion 1.
        // So this way instead of [Numpad5, LAlt] we get [LAlt, Numpad5]
        // Easier to check if ALT is pressed
        let keys: Vec<Keycode> = device_state.get_keys().into_iter().rev().collect();

        if keys != prev_keys {
            if let Some(keycode) = keys.get(0) {
                match *keycode {
                    // Exit the program
                    Keycode::End => process::exit(1),
                    // Alt key pressed and other button
                    Keycode::LAlt | Keycode::RAlt => {
                        // If we have a second key pressed
                        if let Some(next_keycode) = keys.get(1) {
                            change_mouse_listening(next_keycode);
                        }
                    }
                    _ => (),
                }
            }
        }
        prev_keys = keys;
        thread::sleep(Duration::from_millis(50));
    }
}

fn change_mouse_listening(keycode: &Keycode) {
    // Turn auto clicker on/off or the listening for a specific mouse button on/off
    match *keycode {
        Keycode::Numpad1 | Keycode::Key1 => {
            let new_left_state = !USE_LEFT_MOUSE.load(Ordering::Relaxed);
            USE_LEFT_MOUSE.swap(new_left_state, Ordering::Relaxed);

            println!("LEFT mouse button check: {}", check_on_off(new_left_state));
        }
        Keycode::Numpad2 | Keycode::Key2 => {
            let new_right_state = !USE_RIGHT_MOUSE.load(Ordering::Relaxed);
            USE_RIGHT_MOUSE.swap(new_right_state, Ordering::Relaxed);

            println!(
                "RIGHT mouse button check: {}",
                check_on_off(new_right_state)
            );
        }
        Keycode::Numpad3 | Keycode::Key3 => {
            let new_middle_state = !USE_MIDDLE_MOUSE.load(Ordering::Relaxed);
            USE_MIDDLE_MOUSE.swap(new_middle_state, Ordering::Relaxed);

            println!(
                "MIDDLE mouse button check: {}",
                check_on_off(new_middle_state)
            );
        }
        Keycode::Numpad5 | Keycode::Key5 => {
            let new_switch_state = !THE_SWITCH.load(Ordering::Relaxed);
            THE_SWITCH.swap(new_switch_state, Ordering::Relaxed);

            println!("Auto-click: {}", check_on_off(new_switch_state));
        }
        _ => (),
    }
}

fn check_on_off(button_on: bool) -> String {
    if button_on {
        String::from("ON")
    } else {
        String::from("OFF")
    }
}
