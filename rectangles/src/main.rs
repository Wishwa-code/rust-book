fn main() {

    let mut n = 0;
    let mut p = &n;

    loop {
        // println!("This is number {}", n);
        n = n + 1;
        match n{
            100 => break,
            n if n % 5 == 0 && n % 3 == 0 => println!("both"),
            n if n % 5 == 0 => println!("multiple of five!"),
            n if n % 3 == 0 => println!("multiple of three!"),
            1 => println!("one"),
            _ => (), 
        }
    }

}

