fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no pr // BIG PROBLEM

    println!("{}, and {}", r1, r2);
    
}

