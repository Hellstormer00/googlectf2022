use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    ops::RangeBounds,
};

fn main() {
    let equalsre = regex::Regex::new(r"#if S == (\d+)").unwrap();
    let definere = regex::Regex::new(r"#define S (\d+)").unwrap();

    let file = File::open("files/cpp.c").unwrap();
    let reader = BufReader::new(file);

    let mut string_out = String::new();

    let mut tabs = String::new();
    for (n, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut new_line = format!("{}{}\n", tabs, line);

        if line.starts_with("#if") {
            new_line = format!("{}{}\n", tabs, line);
            tabs.push('\t');
        } else if line.starts_with("#else") {
            tabs.pop();
            new_line = format!("{}{}\n", tabs, line);
            tabs.push('\t');
        } else if line.starts_with("#endif") {
            tabs.pop();
            new_line = format!("{}{}\n", tabs, line);
        }

        string_out.push_str(&new_line);

        if let Some(caps) = equalsre.captures(&line) {
            let s = caps.get(1).unwrap().as_str();
            string_out.push_str(&format!("{}#pragma message \"S=={}\"\n", tabs, s))
        }

        if let Some(caps) = definere.captures(&line) {
            let s = caps.get(1).unwrap().as_str();
            string_out.push_str(&format!("{}#pragma message \"#define S {}\"\n", tabs, s))
        }
    }

    let mut out_file = File::create("files/cpp_formatted.c").unwrap();
    out_file.write_all(string_out.as_bytes()).unwrap();
}
