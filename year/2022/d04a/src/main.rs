use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    let mut overlap_pairs = 0;
    for line in input.lines() {
        let split_line: Vec<&str> = Regex::new(r"[-,]").unwrap().split(line).collect();
        let mut parsed_ints: Vec<i32> = Vec::new();

        for num in split_line {
            parsed_ints.push((num.parse::<i32>().unwrap()))
        }

        if (parsed_ints[0] >= parsed_ints[2] && parsed_ints[1] <= parsed_ints[3]) ||
            (parsed_ints[0] <= parsed_ints[2] && parsed_ints[1] >= parsed_ints[3]) {
            overlap_pairs += 1;
        }
    }

    println!("{}", overlap_pairs);
}
