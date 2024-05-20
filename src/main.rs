use std::{
    io::{stdin, stdout, Write},
    num::ParseIntError,
};

fn main() {
    let mut conversion_str = "Binary"; // TODO: Remove debug line
    match choose_conversion() {
        1 => (),
        2 => conversion_str = "Hexadecimal",
        _ => (),
    }
    println!("{}", conversion_str);
}

fn choose_conversion() -> u8 {
    let mut input_str = String::new();

    loop {
        println!("What do you want to convert to:");
        println!("[1] Binary");
        println!("[2] Hexadecimal");
        print!("> ");

        stdout().flush().expect("Error flushing output.");
        match stdin().read_line(&mut input_str) {
            Ok(_) => {
                if !will_parse(input_str.trim().parse::<u8>()) {
                    println!("Not a valid input, try again.\n");
                    input_str = String::from(""); // Reset input
                    continue;
                }
                break;
            }
            Err(error) => {
                println!("Error given: {error}, try again.\n");
                continue;
            }
        };
    }

    let input = input_str.trim().parse::<u8>().unwrap();
    input
}

/// Checks if the string can be parsed to u8 and the parsed number is equal to
/// 1 or 2.
fn will_parse(parse_data: Result<u8, ParseIntError>) -> bool {
    match parse_data {
        Ok(num) => {
            if num == 1 || num == 2 {
                return true;
            }
            false
        }
        Err(_) => false,
    }
}
