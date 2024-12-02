use std::collections::HashMap;
use crate::utils;
use regex::Regex;

pub fn run() {
    println!("Day 1");

    let input: String = utils::load_input(1);
    let lines: Vec<&str> = input.lines().collect();
    let split_space = Regex::new(r"\s+").unwrap();

    let data: Vec<(i32, i32)> = lines
        .into_iter()
        .map(|line| {
            let parts: Vec<i32> = split_space
                .split(line)
                .map(|value| value.parse().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect();

    let mut list1: Vec<i32> = data.iter()
        .map(|(a, _b)| *a).collect();
    let mut list2: Vec<i32> = data.iter()
        .map(|(_a, b)| *b).collect();

    list1.sort_unstable();
    list2.sort_unstable();

    let distance = list1.iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    println!("Part 1: {:?}", distance);

    let mut count_map: HashMap<i32, i32> = HashMap::new();
    let mut similarity_score = 0;

    for i in 0..list1.len() {
        let num1 = list1[i];

        if !count_map.contains_key(&num1) {
            count_map.insert(num1, list2.iter()
                .filter(|&n| *n == num1).count() as i32);
        }

        similarity_score += num1 * count_map.get(&num1).unwrap();
    }

    println!("Part 2: {:?}", similarity_score);
}
