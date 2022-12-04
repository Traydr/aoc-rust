fn main() {
    let input = include_str!("../input.txt");
    let mut max_calories = Vec::new();

    let mut temp_calories: i32 = 0;
    for var in input.lines() {
        if str::is_empty(var) {
            max_calories.push(temp_calories);
            temp_calories = 0;
            continue;
        }

        temp_calories += var.parse().unwrap_or(0);
    }

    max_calories.sort();
    let length = max_calories.len();
    let sum: i32 = max_calories[length - 1] + max_calories[length - 2] + max_calories[length - 3];
    println!("{}", sum);
}
