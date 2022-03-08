use regex::Regex;
use std::{fs::File, io::Read};

fn main() {
    let rom_re = Regex::new(r"ROM_(\d{8})_(\d) (\d)").unwrap();

    let mut file_contents = String::new();
    File::open("files/cpp.c")
        .unwrap()
        .read_to_string(&mut file_contents)
        .unwrap();

    let rom_output = String::new();

    let mut byte_counter;
    let mut counter_before;
    for cap in rom_re.captures_iter(&file_contents) {
        byte_counter = cap[2].parse::<u32>().unwrap();

        if byte_counter == counter_before {
            break;
        }

        if byte_counter < 8 {
            
        } else {
            panic!("ROM doesn't have valid format.")
        }

        counter_before = byte_counter;
    }
}
