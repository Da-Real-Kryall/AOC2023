//this takes in two arguments; the day number and part number
//it runs either part1 or part2 functions in main.rs in dayXX folder

use std::env;

mod solutions;

fn main() {
    // Get the day number and part number from the command line
    let args: Vec<String> = env::args().collect();
    let day = {
        let x = &args[1];
        if x.len() == 1 {
            format!("0{}", x).to_string()
        } else {
            x.to_string()
        }
    };
    let day = day.as_str();

    let part = args[2].as_str();

    // Run the solution
    let solution: String = match day {
        "01" => match part {
            "1" => solutions::day01::part1(),
            "2" => solutions::day01::part2(),
            _ => panic!("Invalid part number"),
        },
        _ => panic!("Invalid day number"),
    };
    // Print the solution
    println!("{}", solution);
}
