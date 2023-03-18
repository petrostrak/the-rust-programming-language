// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use io::Result as ResultIO;
use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        match state {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdow" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_state(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("Turning Off.."),
        Sleep => println!("Sleeping mode activated."),
        Reboot => println!("Rebooting..."),
        Shutdown => println!("Shuting down..."),
        Hibernate => println!("Entering Hibernation"),
    }
}

fn read_input() -> ResultIO<String> {
    valid_commands();
    println!("Give a command:");
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_lowercase())
}

fn valid_commands() {
    println!("----- Power state commands -----");
    println!("Off - turns PC off");
    println!("Sleep - puts PC to sleep mode");
    println!("Reboot - reboots PC");
    println!("Shutdown - shuts PC down");
    println!("Hibernate - puts PC to hibernate mode");
    println!("----- Power state commands -----");
}

fn main() {
    let input = read_input();
    if input.is_ok() {
        match PowerState::new(input.unwrap().as_str()) {
            Some(state) => print_state(state),
            None => println!("Invalid power state command"),
        }
    } else {
        println!("Error reading input");
    }
}
