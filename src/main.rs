use hex_to_binary::ui::{choose_conversion, hex_to_bin};

fn main() {
    match choose_conversion() {
        1 => hex_to_bin(),
        2 => println!("Hexadecimal"),
        _ => (),
    };
}
