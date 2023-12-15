use std::collections::HashMap;
use std::str::Chars;

fn main() {
    let example_1 = "Time:      7  15   30
Distance:  9  40  200";
    let input = include_str!("../../inputs/d06.txt");

    assert_eq!(get_num_ways(example_1), 288);

    println!("Number of Ways: {}", get_num_ways(input));
}

fn get_num_ways(input: &str) -> u32 {
    let mut sum_ways: u32 = 0;
    let mut vec_of_nums: Vec<Vec<u32>> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    for i in 0..lines.len() {
        vec_of_nums.push(
            lines.get(i)
                .unwrap()
                .split(' ')
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect())
    }

    for i in 0..vec_of_nums[0].len() {
        let time: u32 = vec_of_nums[0][i];
        let distance: u32 = vec_of_nums[1][i];
    }

    return sum_ways;
}

fn is_num(string_to_check: Chars) -> bool {
    let mut is_a_number: bool = true;
    for char in string_to_check {
        match char {
            '1' => continue,
            '2' => continue,
            '3' => continue,
            '4' => continue,
            '5' => continue,
            '6' => continue,
            '7' => continue,
            '8' => continue,
            '9' => continue,
            '0' => continue,
            _ => { is_a_number = false }
        }
    }
    return is_a_number;
}