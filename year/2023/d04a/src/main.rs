fn main() {
    let example_1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let example_2 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let input = include_str!("../../inputs/d04.txt");

    assert_eq!(get_points(example_1), 13);
    assert_eq!(get_points(example_2), 8);

    println!("Total Points: {}", get_points(input));
}

fn get_points(input: &str) -> u32 {
    let mut points: u32 = 0;
    for line in input.lines() {
        let card: Vec<&str> = line.split(": ").collect();
        let card_num_str: String = card[0].chars().filter(|c| c.is_numeric()).collect();
        // let card_num: u32 = card_num_str.parse::<u32>().unwrap();

        let numbers: Vec<&str> = card[1].split(" | ").collect();
        let winning_numbers: Vec<&str> = numbers[0].split(" ").collect();
        let possible_numbers: Vec<&str> = numbers[1].split(" ").collect();

        let mut winners: u32 = 0;
        for num in possible_numbers {
            if num.is_empty() { continue; }

            if winning_numbers.contains(&num) {
                winners += 1;
            }
        }

        let temp_points = match winners {
            0 => 0,
            1 => 1,
            num => 2u32.pow(num - 1)
        };

        points += temp_points;
    }

    return points;
}
