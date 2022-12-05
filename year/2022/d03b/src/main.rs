// Offset so 'a' starts at 1
const LOWERCASE_OFFSET: i32 = 96;
// Offset so 'A' starts at 27
const UPPERCASE_OFFSET: i32 = 64 - 26;

fn main() {
    let input = include_str!("../input.txt");

    let mut sum = 0;
    let mut counter = 1;
    let mut last_two_lines: [&str; 2] = ["init"; 2];

    for line in input.lines() {
        if counter % 3 == 0 {
            for c in line.chars() {
                if last_two_lines[0].contains(c) && last_two_lines[1].contains(c) {
                    sum += char_to_int(c);
                    counter += 1;
                    break;
                }
            }
            continue;
        }

        last_two_lines[(counter % 3) - 1] = line;
        counter += 1;
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
