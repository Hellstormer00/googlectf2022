fn main() {
    let equalsre = regex::Regex::new(r"#if S == (\d+)").unwrap();
    println!("{:?}", equalsre.captures("#if S == 13"));
}
