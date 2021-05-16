use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, MouseButton, MouseControllable};

use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, execute, style};
use rand::Rng;
use std::io::stdout;
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

const HELP_MESSAGE: &str = "\nNo arguments found.\n
    Enter a number in milliseconds
    If two numbers are specified, a random number between the two values is used\n
    END key quits the program
    ALT + (Num-5 OR Numpad-5) turns ALL auto click ON/OFF\n
    Default the LEFT mouse button auto click is ON
    ALT + (Num-1 OR Numpad-1) toggle ON/OFF\n
    Default the RIGHT mouse button auto click is OFF
    ALT + (Num-2 OR Numpad-2) toggle ON/OFF\n
    Default the MIDDLE mouse button auto click is OFF
    ALT + (Num-3 OR Numpad-3) toggle ON/OFF\n";

fn main() {
    //Try to enter alternate screen or print error
    if let Err(err_mess) = execute!(stdout(), terminal::EnterAlternateScreen) {
        println!("{}", err_mess);
    }
    if let Ok(_raw) = terminal::enable_raw_mode() {
        clear_term();

        let arguments: Vec<String> = env::args().collect();

        if arguments.len() == 2 {
            handle_two_arguments(&arguments);
        } else if arguments.len() == 3 {
            // We need 2 or 3 arguments
            handle_three_arguments(&arguments);
        } else {
            println!("{}", HELP_MESSAGE);
        }
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
        // Has a default sleep of 50 milliseconds
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
        // Has a default sleep of 50 milliseconds
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

        // Show on screen the state of all the auto mouse clicking
        show_mouse_on_off();

        // Sleep for a random time
        let sleep_time = rand::thread_rng().gen_range(millisec_one..=millisec_two);
        thread::sleep(Duration::from_millis(sleep_time));
    }
}

fn show_mouse_on_off() {
    //Print to screen or give error message if it fails

    // Message ALL mouse buttons
    let all_message = format!(
        "Auto-clicker : {}  [Even if the others below are ON, this will stop ALL auto-clicking] ",
        check_on_off(THE_SWITCH.load(Ordering::Relaxed))
    );
    if let Err(err_mess) = execute!(stdout(), cursor::MoveTo(1, 1), style::Print(all_message)) {
        println!("{}", err_mess);
    }

    let left_message = format!(
        "LEFT-mouse   : {} ",
        check_on_off(USE_LEFT_MOUSE.load(Ordering::Relaxed))
    );
    // Message LEFT mouse button
    if let Err(err_mess) = execute!(stdout(), cursor::MoveTo(1, 2), style::Print(left_message)) {
        println!("{}", err_mess);
    }

    let right_message = format!(
        "RIGHT-mouse  : {} ",
        check_on_off(USE_RIGHT_MOUSE.load(Ordering::Relaxed))
    );
    // Message RIGHT mouse button
    if let Err(err_mess) = execute!(stdout(), cursor::MoveTo(1, 3), style::Print(right_message)) {
        println!("{}", err_mess);
    }

    let middle_message = format!(
        "MIDDLE-mouse : {} ",
        check_on_off(USE_MIDDLE_MOUSE.load(Ordering::Relaxed))
    );
    // Message MIDDLE mouse button
    if let Err(err_mess) = execute!(stdout(), cursor::MoveTo(1, 4), style::Print(middle_message)) {
        println!("{}", err_mess);
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
                    Keycode::End => cleanup_on_exit(),
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
        }
        Keycode::Numpad2 | Keycode::Key2 => {
            let new_right_state = !USE_RIGHT_MOUSE.load(Ordering::Relaxed);
            USE_RIGHT_MOUSE.swap(new_right_state, Ordering::Relaxed);
        }
        Keycode::Numpad3 | Keycode::Key3 => {
            let new_middle_state = !USE_MIDDLE_MOUSE.load(Ordering::Relaxed);
            USE_MIDDLE_MOUSE.swap(new_middle_state, Ordering::Relaxed);
        }
        Keycode::Numpad5 | Keycode::Key5 => {
            let new_switch_state = !THE_SWITCH.load(Ordering::Relaxed);
            THE_SWITCH.swap(new_switch_state, Ordering::Relaxed);
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

//Clear terminal
fn clear_term() {
    //Print to screen or give error message if it fails
    if let Err(err_mess) = execute!(
        stdout(),
        cursor::MoveTo(1, 1),
        terminal::Clear(ClearType::All),
    ) {
        println!("{}", err_mess);
    }
}

//Do all this if we want to exit the program
fn cleanup_on_exit() {
    // print_at_pos(0, cursor::position().unwrap().1, "Quiting the program");

    // disable mouse events to be captured or print error if it fails
    if let Ok(_raw) = terminal::disable_raw_mode() {
        if let Err(error_mess) = execute!(stdout()) {
            println!("{}", error_mess);
        }
    }

    clear_term();

    //Try to leave this screen and go back to the one we started this program in or print error
    if let Err(err_mess) = execute!(stdout(), terminal::LeaveAlternateScreen) {
        println!("{}", err_mess);
    }

    //Stop the program
    std::process::exit(1);
}
