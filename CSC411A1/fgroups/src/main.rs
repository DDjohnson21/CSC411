use std::io;

fn main() {
    // println!("Hello, world!");
    
    // let input = env::args().nth(1);
    // assert!(env::args().len() == 2);

    //line begins with one or more non-whitespace characters = Fingerprint - has no spaces 

    // after fingerprint,there are one or more whitespace characters not including newlines --- has spaces 

    // The name begins with the next non-whitespace character and continues 
    //through the next newline. A name may contain whitespace, but a name never contains a newline

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
    let input = input.trim();

    // Split the input into two parts, ignoring multiple spaces
    let parts: Vec<&str> = input.split_whitespace().collect(); 

    // Check if we have at least two parts
    if parts.len() >= 2 {
        let first_part = parts[0];
        let second_part = parts[1..].join(" "); // Join the remaining parts

        println!("First part: {}", first_part);
        println!("Second part: {}", second_part);
    } 
}