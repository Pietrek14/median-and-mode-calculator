use std::io;

fn input_u32(input_message: &str) -> u32 {
    let result = loop {
        println!("{}", input_message);

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read from stdin!");

        input = input.trim().to_string();

        let cast = input.parse::<u32>();

        match cast {
            Ok(result) => {
                break result;
            },
            Err(_) => {
                println!("The value entered is not a number!");
            }
        }
    };

    result
}

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

    println!("Median: {}", median)
}
