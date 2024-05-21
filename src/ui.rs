use std::{
    io::{stdin, stdout, Write},
    num::ParseIntError,
};

use crate::conversion::{binary_to_hex_conversion, decimal_to_bin, parse_binary, parse_hex};

pub fn choose_conversion() -> u8 {
    let mut input_str = String::new();

    loop {
        println!("What do you want to convert to:");
        println!("[1] Binary");
        println!("[2] Hexadecimal");
        print!("> ");

        stdout().flush().expect("Error flushing output.");
        match stdin().read_line(&mut input_str) {
            Ok(_) => {
                if !will_parse_option_input(input_str.trim().parse::<u8>()) {
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
fn will_parse_option_input(parse_data: Result<u8, ParseIntError>) -> bool {
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

#[allow(unused_assignments)]
pub fn hex_to_bin() {
    let mut hex_input = String::new();
    let mut binary = String::new();

    loop {
        print!("Hex number: ");

        stdout().flush().expect("Error flushing output.");
        match stdin().read_line(&mut hex_input) {
            Ok(_) => {
                match parse_hex(hex_input.clone()) {
                    Ok(hex_as_decimal) => {
                        binary = decimal_to_bin(hex_as_decimal);
                        break;
                    }
                    Err(_) => {
                        println!("Invalid input, try again.");
                        hex_input = String::from(""); // Reset input
                        continue;
                    }
                };
            }
            Err(error) => {
                println!("Error given: {error}, try again.\n");
                continue;
            }
        }
    }

    println!("{}", binary);
}

#[allow(unused_assignments)]
pub fn binary_to_hex() {
    let mut binary_input = String::new();
    let mut hex = String::new();

    loop {
        print!("Binary number: ");

        stdout().flush().expect("Error flushing output");
        match stdin().read_line(&mut binary_input) {
            Ok(_) => match parse_binary(binary_input.clone()) {
                Ok(bits) => {
                    hex = binary_to_hex_conversion(bits);
                    break;
                }
                Err(_) => {
                    println!("Invalid input, try again");
                    binary_input = String::from(""); // Reset input
                    continue;
                }
            },
            Err(error) => {
                println!("Error given: {error}, try again.\n");
                continue;
            }
        }
    }

    println!("{}", hex);
}
