fn main() {
    let input_test =
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
    let input = include_str!("../../inputs/d01.txt");
    let nums_as_int = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let nums_as_str = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero"];
    let mut total_sum: u32 = 0;

    for line in input.lines() {
        // Get the nums
        let mut only_nums: Vec<char> = vec![];
        let mut last_chars: Vec<char> = vec![];
        for c in line.chars() {
            if c == ' ' {
                continue
            }
            last_chars.push(c);
            if nums_as_int.contains(&c) {
                only_nums.push(c);
                continue;
            }

            let mut possible_nums: Vec<&str> = vec![];
            for word in nums_as_str.iter() {
                if word.ends_with(c) {
                    possible_nums.push(word)
                }
            }

            for str_num in possible_nums {
                // println!("Analysing {:}", str_num);
                if last_chars.len() < str_num.len() {
                    continue;
                }
                let comparison= last_chars.windows(str_num.len()).last().unwrap().to_vec();
                let comparison_str = comparison.iter().collect::<String>();
                if comparison_str == str_num {
                    let parsed_int: char = match str_num {
                        "one" => '1',
                        "two" => '2',
                        "three" => '3',
                        "four" => '4',
                        "five" => '5',
                        "six" => '6',
                        "seven" => '7',
                        "eight" => '8',
                        "nine" => '9',
                        "zero" => '0',
                        _ => ' '
                    };

                    only_nums.push(parsed_int);
                    break
                }
            }
        }

        if only_nums.len() == 0 {
            panic!("Nums remaining cannot be 0")
        }

        // Add the numbers together
        let mut str_nums: String = String::new();
        str_nums.push(only_nums[0]);

        if only_nums.len() == 1 {
            str_nums.push(only_nums[0]);
        } else {
            str_nums.push(only_nums.last().unwrap().to_ascii_lowercase());
        }

        let num: u32 = str_nums.parse().unwrap();
        total_sum += num;
    }

    println!("Total Sum: {:}", total_sum)
}
