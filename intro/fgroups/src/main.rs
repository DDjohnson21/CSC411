use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in stdin.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            eprintln!("Badly formed input: {}", line);
            continue;
        }

        let fingerprint = parts[0].to_string();
        if fingerprint.len() > 512 {
            eprintln!("Fingerprint too long: {}", fingerprint);
            continue;
        }

        map.entry(fingerprint)
            .or_insert_with(Vec::new)
            .push(parts[1..].join(" ").to_string());
    }

    let mut counter = 0;
    for value in map.values() {
        if value.len() > 1 {
            if counter != 0 {
                println!(); // Print an empty line to separate groups
            }
            for element in value {
                println!("{}", element);
            }
            counter += 1; // Increment the counter to manage the separation of groups
        }
    }
}