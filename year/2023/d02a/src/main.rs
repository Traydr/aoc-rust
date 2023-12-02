fn main() {
    // Inputs
    let _input_test =
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
         Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
         Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
         Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
         Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let _input = include_str!("../../inputs/d02.txt");

    // Change this as determined by the challenge
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    // Output
    let mut sum_games: u32 = 0;

    for line in _input.lines() {
        let mut current_red: u32 = 0;
        let mut current_green: u32 = 0;
        let mut current_blue: u32 = 0;
        let mut game_num: u32 = 0;

        // Get game number
        let game: Vec<&str> = line.split(":").collect();
        let mut game_str = String::from("");

        for c in game[0].chars() {
            if c.is_numeric() {
                game_str.push(c);
            }
        }

        game_num = game_str.parse().unwrap();

        // Get data from rounds
        let rounds: Vec<&str> = game[1].split(";").collect();
        let rounds_len = rounds.len();
        let mut rounds_correct: usize = 0;

        for round in rounds {
            let colors: Vec<&str> = round.split(",").collect();
            for color in colors {
                let temp_line: Vec<&str> = color.split(" ").collect();
                let temp_num: u32 = temp_line[1].parse().unwrap();
                match temp_line[2] {
                    "red" => current_red += temp_num,
                    "green" => current_green += temp_num,
                    "blue" => current_blue += temp_num,
                    _ => {panic!("There should be a color")}
                }
            }

            if !(current_red > max_red || current_green > max_green || current_blue > max_blue) {
                rounds_correct += 1;
            }

            current_red = 0;
            current_green = 0;
            current_blue = 0;
        }



        if rounds_correct == rounds_len {
            sum_games += game_num;
        }
    }

    println!("Sum games: {:}", sum_games)
}
