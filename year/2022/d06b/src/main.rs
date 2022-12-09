fn main() {
    let input: Vec<char> = include_str!("../input.txt").chars().collect();
    const START_FROM_CHAR: usize = 13;

    let mut stack: Vec<char>;
    for i in START_FROM_CHAR..input.len() - 1 {
        stack = Vec::from_iter(input[i - START_FROM_CHAR..i + 1].iter().cloned());

        let mut duplicates = 0;
        for char_one in stack.clone() {
            for char_two in stack.clone() {
                if char_one == char_two {
                    duplicates += 1;
                }
            }
        }

        if duplicates > 14 {
            continue;
        }

        println!("{}", i + 1);
        break;
    }
}
