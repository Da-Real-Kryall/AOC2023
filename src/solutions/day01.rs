// Day 1 solution by Da-Real-Kryall


pub fn part1() -> String {

    let numbers: String = "1234567890".to_string();

    let input = include_str!("../inputs/day01.txt");
    let mut current_total: u32 = 0;
    for line in input.lines() {
        //get first digit
        let mut index = 0;
        //while the nth digit is not a number
        while !numbers.contains(line.chars().nth(index).unwrap()) {
            index = index + 1;
        }
        let first_digit: u32 = line.chars().nth(index).unwrap().to_digit(10).unwrap();
        //get last digit
        index = line.len()-1;
        while !numbers.contains(line.chars().nth(index).unwrap()) {
            index = index - 1;
        }
        let last_digit: u32 = line.chars().nth(index).unwrap().to_digit(10).unwrap();
        //get the number
        let number: u32 = first_digit*10+last_digit;
        println!("{} {} {}", first_digit, last_digit, number);
        current_total += number;
    }
    current_total.to_string()
}

//208191
pub fn part2() -> String {

    let replacements: Vec<(&'static str, &'static str)> = vec![
        ("zero", "zer0ero"),
        ("one", "on1ne"),
        ("two", "tw2wo"),
        ("three", "thre3hree"),
        ("four", "fou4our"),
        ("five", "fiv5ive"),
        ("six", "si6ix"),
        ("seven", "seve7even"),
        ("eight", "eigh8ight"),
        ("nine", "nin9ine")
    ];
    let numbers: String = "1234567890".to_string();
    let input = include_str!("../inputs/day01.txt");
    let mut current_total: u32 = 0;
    for line in input.lines() {
        //replace words with numbers
        let mut new_line = line.to_string();
        for replacement in replacements.iter() {
            new_line = new_line.replace(replacement.0, replacement.1);
        }
        println!("{} -> {}", line, new_line);
        //get first digit
        let mut index = 0;
        //while the nth digit is not a number
        while !numbers.contains(new_line.chars().nth(index).unwrap()) {
            index = index + 1;
        }
        let first_digit: u32 = new_line.chars().nth(index).unwrap().to_digit(10).unwrap();
        //get last digit
        index = new_line.len()-1;
        while !numbers.contains(new_line.chars().nth(index).unwrap()) {
            index = index - 1;
        }
        let last_digit: u32 = new_line.chars().nth(index).unwrap().to_digit(10).unwrap();
        //get the number
        let number: u32 = first_digit*10+last_digit;
        println!("{} {} {}", first_digit, last_digit, number);
        current_total += number;

    }
    current_total.to_string()
}
