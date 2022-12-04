fn main() {
    let input = include_str!("../input.txt");
    let mut max_calories: u32 = 0;

    let mut temp_calories: u32 = 0;
    for var in input.lines() {
        if str::is_empty(var) {
            if temp_calories > max_calories { max_calories = temp_calories }
            temp_calories = 0;
            continue;
        }

        temp_calories += var.parse().unwrap_or(0);
    }
    println!("{}", max_calories)
}
