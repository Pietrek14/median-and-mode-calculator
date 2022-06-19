use std::io;
use std::collections::HashMap;

fn main() {
    let mut numbers: Vec<u32> = vec!();

    loop {
        println!("Enter next number (or nothing to stop input): ");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read from stdin!");

        input = input.trim().to_string();

        match input.as_str() {
            "" => break,
            _ => {
                let cast = input.parse::<u32>();

                match cast {
                    Ok(result) => {
                        numbers.push(result);
                    },
                    Err(_) => {
                        println!("The value entered is not a number!");
                    }
                }
            }
        }
    }

    if numbers.is_empty() {
        println!("No numbers were given!");
        return;
    }

    numbers.sort();

    let median = if numbers.len() % 2 == 0 {
        (numbers[(numbers.len() - 1) / 2] + numbers[(numbers.len() + 1) / 2]) as f32 / 2.
    } else {
        numbers[numbers.len() / 2] as f32
    };

    println!("Median: {}", median);

    let mut element_count:HashMap<u32, u32> = HashMap::new();

    for number in numbers {
        let count = element_count.entry(number).or_insert(0);

        *count += 1;
    }

    let mut mode = element_count.values().next().unwrap();

    for (number, count) in &element_count {
        let max = element_count.get(&mode).unwrap();

        if count > max {
            mode = number;
        }
    }

    println!("Mode: {}", mode)
}
