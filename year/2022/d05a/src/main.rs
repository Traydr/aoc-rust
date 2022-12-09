fn main() {
    let input = include_str!("../input.txt");
    let mut stack_input: Vec<&str> = Vec::new();
    let mut stacks: Vec<Vec<char>>;
    let mut num_stacks = 0;

    for line in input.lines() {
        if str::is_empty(line) { continue; }

        if line.chars().nth(0).unwrap() == '[' {
            println!("Pushing line to stack: {}", line);
            stack_input.push(line);
            continue;
        } else if line.chars().nth(1).unwrap() == '1' {
            println!(" Num Line: {}", line);
            num_stacks = line.split(" ").into_iter().last().unwrap().parse::<i32>().unwrap();

            stacks = Vec::with_capacity(num_stacks as usize);
            for _i in 1..num_stacks {
                stacks.push(Vec::new());
            }

            stack_input.reverse();
            for i in 0..num_stacks {
                let stack_input_line = stack_input.pop().unwrap();
            }
        }
    }
}
