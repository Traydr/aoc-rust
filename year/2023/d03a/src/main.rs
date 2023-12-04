fn main() {
    let _input_test_1 =
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let _input_test_2 = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
    let _input_test_3 = ".......5......
..7*..*.......
...*13*.......
.......15.....";
    let _input_test_4 = ".......5......
..7*..*.....4*
...*13*......9
.......15.....
..............
..............
..............
..............
..............
..............
21............
...*9.........";
    let _input = include_str!("../../inputs/d03.txt");
    let mut schematic: Vec<Vec<char>> = vec![];

    for line in _input.lines() {
        schematic.push(line.chars().collect());
    }

    let mut part_sum: u32 = 0;
    for i in 0..schematic.len() {
        let mut temp_num = String::from("");
        let mut is_part = false;
        for j in 0..schematic[0].len() {
            let current_char = schematic[i][j];
            if !current_char.is_numeric() {
                if temp_num.is_empty() {
                    continue;
                }

                if is_part {
                    println!("Adding {temp_num} to sum");
                    part_sum += temp_num.parse::<u32>().unwrap();
                }

                // Debug only
                if !is_part{
                    println!("Throwing away {temp_num}");
                }

                temp_num = String::from("");
                is_part = false;
                continue;
            }

            temp_num.push(current_char);

            for (k, o) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                let Some(some_vec) = schematic.get((i as isize + k) as usize) else {
                    continue;
                };

                let Some(some_char) = some_vec.get((j as isize + o) as usize) else {
                    continue;
                };

                if is_valid_symbol(some_char) {
                    is_part = true;
                }
            }

            if (j == (schematic[0].len() - 1)) && is_part {
                println!("Adding Edge {temp_num}");
                part_sum += temp_num.parse::<u32>().unwrap();
            }
        }
    }

    println!("Sum of Parts {part_sum}")
}

fn is_valid_symbol(c: &char) -> bool {
    match c {
        '.' => false,
        '1' => false,
        '2' => false,
        '3' => false,
        '4' => false,
        '5' => false,
        '6' => false,
        '7' => false,
        '8' => false,
        '9' => false,
        '0' => false,
        _ => true
    }
}