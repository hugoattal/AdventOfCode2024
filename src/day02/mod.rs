use crate::utils;
use regex::Regex;

pub fn run() {
    println!("Day 2");

    let input: String = utils::load_input(2);
    let lines: Vec<&str> = input.lines().collect();
    let split_space = Regex::new(r"\s+").unwrap();

    let data: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| {
            split_space
                .split(line)
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();

    let safe_list: Vec<bool> = data
        .iter()
        .map(|line| {
            let direction = (line[1] - line[0]).signum();

            for i in 1..line.len() {
                if (line[i] - line[i - 1]).signum() != direction
                    || (line[i] - line[i - 1]).abs() > 3
                {
                    return false;
                }
            }
            return true;
        })
        .collect();

    let total_safe = safe_list.iter().filter(|&x| *x).count();

    println!("Part 1: {:?}", total_safe);

    let safe_list2: Vec<bool> = data
        .iter()
        .map(|line| {
            is_safe(line, true)
        })
        .collect();

    let total_safe2 = safe_list2.iter().filter(|&x| *x).count();

    println!("Part 2: {:?}", total_safe2);
}

fn is_safe(line: &Vec<i32>, life: bool) -> bool {
    let direction = ((line[1] - line[0]).signum()
        + (line[2] - line[1]).signum()
        + (line[3] - line[2]).signum())
    .signum();

    for i in 1..line.len() {
        if (line[i] - line[i - 1]).signum() != direction || (line[i] - line[i - 1]).abs() > 3 {
            if !life {
                return false;
            }
            return is_safe(&exclude_index(line, i), false) || is_safe(&exclude_index(line, i - 1), false);
        }
    }
    true
}

fn exclude_index<T: Clone>(vec: &Vec<T>, index: usize) -> Vec<T> {
    vec.iter()
        .enumerate()
        .filter(|&(i, _)| i != index)
        .map(|(_, val)| val.clone())
        .collect()
}