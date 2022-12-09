fn main() {
    let input: Vec<char> = include_str!("../input.txt").chars().collect();

    let mut stack: Vec<char>;
    for i in 3..input.len() - 1 {
        stack = Vec::from_iter(input[i - 3..i + 1].iter().cloned());

        let mut duplicates = 0;
        for char_one in stack.clone() {
            for char_two in stack.clone() {
                if char_one == char_two {
                    duplicates += 1;
                }
            }
        }

        if duplicates > 4 {
            continue;
        }

        println!("{}", i + 1);
        break;
    }
}
