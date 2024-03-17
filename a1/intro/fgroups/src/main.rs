use std::io;
use std::collections::HashMap;

fn main() {
    // assign stdin to a variable to access later on
    let stdin = io::stdin();
    // Creates a new hashmap to store the fingerprintds and their assciated values
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    // Iterate through stdin
    for line in stdin.lines() {
        let line = line.expect("Failed to read line");
        // Break up the line based on whitespace
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Error handling for bad format
        if parts.len() < 2 {
            eprintln!("Badly formed input: {}", line);
            continue;
        }

        // Assign fingerprint 
        let fingerprint = parts[0].to_string();

        // Error handling for input that is too long
        if fingerprint.len() > 512 {
            eprintln!("Fingerprint too long: {}", fingerprint);
            continue;
        }

        // Place both fingerprint (key) and names (values) into hash map
        map.entry(fingerprint)
            .or_insert_with(Vec::new)
            // Join the rest of the parts of the line
            .push(parts[1..].join(" ").to_string());
    }

    // Printing output in correct format
    let mut counter = 0;
    for value in map.values() {
        if value.len() > 1 {
            if counter != 0 {
                // Print an empty line to separate groups
                println!(); 
            }
            for element in value {
                println!("{}", element);
            }
            // Increment the counter to manage the separation of groups
            counter += 1; 
        }
    }
}