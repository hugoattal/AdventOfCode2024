use std::fs;

pub fn load_input(day: i32) -> String {
    fs::read_to_string(format!("src/day{:02}/input.txt", day)).unwrap()
}