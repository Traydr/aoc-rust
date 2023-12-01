fn main() {
    let input_test =
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
    let input = include_str!("../../inputs/d01.txt");
    let nums = vec!('1', '2', '3', '4', '5', '6', '7', '8', '9', '0');
    let mut total_sum: u32 = 0;

    for line in input.lines() {
        let only_nums: Vec<char> = line.chars().filter(|c| nums.contains(c)).collect();
        if only_nums.len() == 1 {
            let mut str_nums: String = String::new();
            str_nums.push(only_nums[0]);
            str_nums.push(only_nums[0]);

            let num: u32 = str_nums.parse().unwrap();
            total_sum += num;
            // println!("Current Total: {:?} \t Num Added: {:?}", total_sum, num)
        } else {
            let mut str_nums: String = String::new();
            str_nums.push(only_nums[0]);
            str_nums.push(only_nums.last().unwrap().to_ascii_lowercase());

            let num: u32 = str_nums.parse().unwrap();
            total_sum += num;
            // println!("Current Total: {:?} \t Num Added: {:?}", total_sum, num)
        }
        // println!("{:?}", only_nums)
    }

    println!("Total Sum: {:?}", total_sum)
}
