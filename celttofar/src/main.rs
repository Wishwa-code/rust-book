use std::io;

fn main() {
    loop {
        println!("Please select an option:");
        println!("1. Convert Fahrenheit to Celsius");
        println!("2. Convert Celsius to Fahrenheit");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter temperature in Fahrenheit:");
                let mut fahrenheit = String::new();
                io::stdin()
                    .read_line(&mut fahrenheit)
                    .expect("Failed to read line");

                let fahrenheit: f64 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("{:.2} Fahrenheit is {:.2} Celsius", fahrenheit, celsius);
            }
            2 => {
                println!("Enter temperature in Celsius:");
                let mut celsius = String::new();
                io::stdin()
                    .read_line(&mut celsius)
                    .expect("Failed to read line");

                let celsius: f64 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{:.2} Celsius is {:.2} Fahrenheit", celsius, fahrenheit);
            }
            3 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}