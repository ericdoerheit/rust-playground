use std::fs;

use regex::Regex;

fn read_day01_input() -> Vec<i32> {
    // read day 01 input from file
    let day01_input_str = fs::read_to_string("src/day01.txt")
        .expect("Something went wrong reading the file");

    let mut day01_input = Vec::new();

    let lines = day01_input_str.split("\r\n");
    for line in lines {
        let value = line.parse::<i32>().unwrap();
        day01_input.push(value);
    }

    day01_input
}

pub fn day01() {
    println!("\n2020 - Day 01");
    println!("Task: Find the two entries that sum to 2020 \
              and then multiply those two numbers together.");

    let day01_input = read_day01_input();

    let input_len = day01_input.len();
    for i1 in 0..input_len {
        let v1 = day01_input[i1];
        for i2 in i1 + 1..input_len {
            let v2 = day01_input[i2];
            if v1 + v2 == 2020 {
                println!("{} + {} = {} → {} x {} = {}", v1, v2, v1 + v2, v1, v2, v1 * v2);
            }
        }
    }

    println!("Task: Repeat the task of day 01, but now three \
             instead of two values must add up to 2020 \
             (e.g., 979 + 366 + 675 = 2020 → 979 * 366 * 675 = 241861950).");

    for i1 in 0..input_len {
        let v1 = day01_input[i1];
        for i2 in i1 + 1..input_len {
            let v2 = day01_input[i2];
            for i3 in i2..input_len {
                let v3 = day01_input[i3];
                if v1 + v2 + v3 == 2020 {
                    println!("{v1} + {v2} + {v3} = {sum} → {v1} x {v2} x {v3} = {product}",
                             v1 = v1, v2 = v2, v3 = v3, sum = v1 + v2 + v3, product = v1 * v2 * v3);
                }
            }
        }
    }
}

pub fn day02() {
    println!("\n2020 - Day 02");

    // read day 01 input from file
    let day02_input_str = fs::read_to_string("src/day02.txt")
        .expect("Something went wrong reading the file");

    // let mut day02_input = Vec::new();

    // regex to parse one line of the day 02 input
    let re = Regex::new(r"(?m)(?P<num1>[0-9]*)-(?P<num2>[0-9]*) (?P<char>[a-z]): (?P<password>[a-z]*)").unwrap();
    // let regex = Regex::new(r"(?:[0-9]*)-(?:0-9]*) (?:[a-z]): (?:[a-z]*)").unwrap();

    let mut total = 0;
    let mut task1_counter = 0;
    let mut task2_counter = 0;

    let lines = day02_input_str.split("\r\n");
    for line in lines {
        total += 1;
        match re.captures(line) {
            Some(x) => {
                let num1 = x.name("num1").map_or(-1, |m| m.as_str().parse::<i32>().unwrap());
                let num2 = x.name("num2").map_or(-1, |m| m.as_str().parse::<i32>().unwrap());
                let char = x.name("char").map_or("", |m| m.as_str());
                let password = String::from(x.name("password").map_or("", |m| m.as_str()));

                let occurrences = password.matches(char).count() as i32;
                let task1_valid = (occurrences >= num1) && (occurrences <= num2);
                if task1_valid {
                    task1_counter += 1;
                }

                let task2_valid: bool;
                let password_len = password.len() as i32;
                if (num1 > password_len) || (num2 > password_len) {
                    panic!()
                } else {
                    let i1 = (num1 - 1) as usize;
                    let i2 = (num1) as usize;
                    let pos1_char = &password[i1..i2];

                    let i1 = (num2 - 1) as usize;
                    let i2 = (num2) as usize;
                    let pos2_char = &password[i1..i2];

                    task2_valid = ((pos1_char == char) && (pos2_char != char)) ||
                        (pos1_char != char) && (pos2_char == char);
                }

                if task2_valid {
                    task2_counter += 1;
                }
            }
            None => unreachable!()
        }
    }

    println!("Task 1: {} passwords of {} are valid.", task1_counter, total);
    println!("Task 2: {} passwords of {} are valid.", task2_counter, total);
}