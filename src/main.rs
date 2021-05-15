use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use enigo::{Enigo, MouseButton, MouseControllable};

use std::sync::{Arc, Mutex, Once};
use rand::Rng;
use std::time::Duration;
use std::{env, process, thread};
use std::sync::atomic::{AtomicBool, Ordering};

static THE_SWITCH: AtomicBool = AtomicBool::new(true);

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() == 2 {
        handle_two_arguments(&arguments);
    } else if arguments.len() == 3 {
        // We need 2 or 3 arguments
        handle_three_arguments(&arguments);
    } else {
        println!("\nNo arguments found.");
        println!("Enter a number in milliseconds\n");
        println!("If two numbers are specified, a random number between the two values is used");
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
            if mouse.button_pressed[1] {
                enigo.mouse_down(MouseButton::Left);
                println!("{:?}", mouse);
            } else if mouse.button_pressed[2] {
                enigo.mouse_down(MouseButton::Right);
                println!("{:?}", mouse);
            } else if mouse.button_pressed[3] {
                enigo.mouse_down(MouseButton::Middle);
                println!("{:?}", mouse);
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
        // Check the keys so we can exit the program when needed
        let keys = device_state.get_keys();
        if keys != prev_keys {
            if let Some(keycode) = keys.get(0) {
                // Just some random chosen keys
                if *keycode == Keycode::Numpad5 || *keycode == Keycode::End {
                    std::process::exit(1);
                }else if *keycode == Keycode::Numpad0 {
                    let new_switch_state = !THE_SWITCH.load(Ordering::Relaxed);
                    THE_SWITCH.swap(new_switch_state, Ordering::Relaxed);

                    if new_switch_state {
                        println!("Squeak-OFF");
                    }else {
                        println!("Squeak-ON");
                    }
                }
            }
        }
        prev_keys = keys;
        thread::sleep(Duration::from_millis(50));
    }
}
