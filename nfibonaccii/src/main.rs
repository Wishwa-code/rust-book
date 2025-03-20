use std::io;

fn main() {
    loop {
        println!("Please select an option:");
        println!("1. Generate nth Fibonacci number");
        println!("2. Exit");

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
                println!("Enter the value of n:");
                let mut n = String::new();
                io::stdin()
                    .read_line(&mut n)
                    .expect("Failed to read line");

                let n: u32 = match n.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let fib_num = fibonacci(n);
                println!("The {}th Fibonacci number is: {}", n, fib_num);
            }
            2 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
