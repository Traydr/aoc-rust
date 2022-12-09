use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let num_stacks: i32 = 9;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks as usize);

    // Setup
    // Add data to vectors
    stacks.push(vec!['C', 'Z', 'N', 'B', 'M', 'W', 'Q', 'V']);
    stacks.push(vec!['H', 'Z', 'R', 'W', 'C', 'B']);
    stacks.push(vec!['F', 'Q', 'R', 'J']);
    stacks.push(vec!['Z', 'S', 'W', 'H', 'F', 'N', 'M', 'T']);
    stacks.push(vec!['G', 'F', 'W', 'L', 'N', 'Q', 'P']);
    stacks.push(vec!['L', 'P', 'W']);
    stacks.push(vec!['V', 'B', 'D', 'R', 'G', 'C', 'Q', 'J']);
    stacks.push(vec!['Z', 'Q', 'N', 'B', 'W']);
    stacks.push(vec!['H', 'L', 'F', 'C', 'G', 'T', 'J']);

    // Parse lines
    for line in input.lines() {
        if str::is_empty(line) || line.chars().nth(0).unwrap() != 'm' { continue; }
        let split_line: Vec<&str> = Regex::new(r"move | from | to ").unwrap().split(line).collect();

        // Sort split_line into readable vars
        let num_items_to_move: i32 = split_line[1].parse().unwrap();
        let mut move_from_stack: usize = split_line[2].parse().unwrap();
        let mut move_to_stack: usize = split_line[3].parse().unwrap();

        // Adjust by -1 so they match the above vec stacks
        move_from_stack -= 1;
        move_to_stack -= 1;

        let mut temp_stack: Vec<char> = Vec::new();
        for _i in 0..num_items_to_move {
            let temp_move = stacks[move_from_stack].pop().unwrap();
            temp_stack.push(temp_move);
        }

        temp_stack.reverse();

        for c in temp_stack {
            stacks[move_to_stack].push(c);
        }
    }

    //  Output
    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }
}
