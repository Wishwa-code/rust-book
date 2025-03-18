use std::io;
use std::collections::HashMap;

fn main() {

    println!("Enter list of intergers separated by white space");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut integers: Vec<i32> = Vec::new();
    let mut frequency_map: HashMap<i32, u32> = HashMap::new();

    for s in input.split_whitespace(){
        let num: i32 = s.parse().expect("Please enter valid interger");
        integers.push(num);

        let count = frequency_map.entry(num).or_insert(0);
        *count += 1;
    }
    
    let mode = frequency_map
    .iter()
    .max_by_key(|&(_, count)| count)
    .map(|(&num, _)| num);



    let median = calculate_median(&mut integers); 
    println!("Median of {:?} is {:?}",integers, median);

    match mode {
        Some(m) => println!("Mode of the vector: {}", m),
        None => println!("No mode found (vector is empty)."),
    }

    // println!("The first element is: {first}");

}

fn calculate_median(integers: &mut Vec<i32>) -> i32 {
    integers.sort();

    let length = integers.len();

    match length % 2{
        0 => {
            let mid1 = integers[length/2 -1];
            let mid2  = integers[length/2];
            (mid1 + mid2) as i32 / 2
        }
        _ => {
            integers[length / 2] as i32 
        }
    }

}