use std::fmt::Error;

use crate::HEX_AS_DECIMAL;

pub fn parse_hex(hex_input: String) -> Result<Vec<&'static str>, Error> {
    let characters: Vec<char> = hex_input
        .to_uppercase()
        .strip_suffix("\n")
        .expect("No \n")
        .chars()
        .collect();

    let mut chars_as_decimal: Vec<&'static str> = Vec::new();
    for character in characters {
        match HEX_AS_DECIMAL.get(&character) {
            Some(value) => {
                chars_as_decimal.push(value);
            }
            None => return Err(Error),
        }
    }

    Ok(chars_as_decimal)
}

pub fn decimal_to_bin(decimal_values: Vec<&'static str>) -> String {
    let mut binary = String::new();

    for num_str in decimal_values {
        let mut num = num_str.parse::<u8>().unwrap();

        let mut value = 8;
        loop {
            if num / value == 1 || num == value {
                binary.push_str("1");
                num -= value;
            } else {
                binary.push_str("0");
            }

            if value == 1 {
                break;
            }
            value /= 2;
        }
        binary.push_str(" ");
    }

    binary
}

pub fn parse_binary(binary_input: String) -> Result<Vec<char>, Error> {
    let mut characters: Vec<char> = binary_input
        .strip_suffix("\n")
        .expect("No \n")
        .chars()
        .rev()
        .peekable()
        .collect();

    for character in &characters {
        if *character != '0' && *character != '1' {
            return Err(Error);
        }
    }

    while characters.len() % 4 != 0 {
        characters.push('0');
    }

    characters.reverse();

    Ok(characters)
}

pub fn binary_to_hex_conversion(bits: Vec<char>) -> String {
    let mut hex = String::new();
    let mut current_byte: u8 = 0;
    let mut bit_place = 8;

    for bit_char in bits {
        let bit = bit_char.to_string().parse::<u8>().unwrap();

        if bit == 1 {
            current_byte += bit_place;
        }

        if bit_place == 1 {
            HEX_AS_DECIMAL.into_iter().find_map(|(key, &val)| {
                if val.parse::<u8>().unwrap() == current_byte {
                    hex.push_str(&key.to_string());
                    Some(())
                } else {
                    None
                }
            });
            bit_place = 8;
            current_byte = 0;
            continue;
        }

        bit_place /= 2;
    }

    hex
}
