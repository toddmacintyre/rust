use std::io;
// use std::cmp::Ordering;

fn get_conversion() -> String {
    loop {
        println!("Which temperature would you like to convert?");
        println!("Fahrenheit -> Celsius (1)");
        println!("Celsius -> Fahrenheit (2)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        match input.trim().parse::<u8>() {
            Ok(num) => match num {
                n @ 1..=2 => match n {
                    1 => break String::from("Fahrenheit"),
                    2 => break String::from("Celsius"),
                    _ => unreachable!(),
                },
                _ => {
                    println!("Please enter the numbers 1 or 2.");
                    continue;
                }
            },
            Err(_) => {
                println!("Number was not of type u8.");
                continue;
            }
        };
    }
}

fn get_temperature(original: &String) -> (i32, i32) {
    println!("Please enter a temperature:");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        let input: i32 = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parse input into i32");
                continue;
            }
        };

        if original == "Fahrenheit" {
            break (input, convert_to_celsius(input));
        }
        break (input, convert_to_fahrenheit(input));
    }
}

fn convert_to_celsius(degrees_fahrenheit: i32) -> i32 {
    // T(°C) = (T(°F) - 32) × 5/9
    (degrees_fahrenheit - 32) * 5 / 9
}

fn convert_to_fahrenheit(degrees_celsius: i32) -> i32 {
    // T(°F) = T(°C) × 9/5 + 32
    degrees_celsius * 9 / 5 + 32
}

fn main() {
    let original_metric = get_conversion();
    let converted_metric = if original_metric == "Celsius" {
        "Fahrenheit"
    } else {
        "Celsius"
    };
    let (original_temp, converted_temp) = get_temperature(&original_metric);
    println!(
        "{original_temp} {original_metric} is equal to {converted_temp} {converted_metric}",
        original_temp = original_temp,
        original_metric = original_metric,
        converted_temp = converted_temp,
        converted_metric = converted_metric,
    );
}
