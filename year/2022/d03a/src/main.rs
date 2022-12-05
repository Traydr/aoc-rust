const LOWERCASE_OFFSET: i32 = 96;
const UPPERCASE_OFFSET: i32 = 64 - 26;

fn main() {
    let input = include_str!("../input.txt");

    let mut sum = 0;

    for line in input.lines() {
        let length = line.len();
        let halves = line.clone().split_at(length/2);

        for c in halves.0.chars() {
            if halves.1.contains(c) {
                println!("Char {} is int {}", c, char_to_int(c));
                sum += char_to_int(c);
                break;
            }
        }
    }

    println!("{}", sum);
}

fn char_to_int(c: char) -> i32 {
    if c.is_lowercase() {
        return (c as i32) - LOWERCASE_OFFSET;
    } else {
        return (c as i32) - UPPERCASE_OFFSET;
    }
}