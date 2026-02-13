use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: notesanitizer <input_file>");
        return;
    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Could not read file");

    let mut seen = HashSet::new();
    let mut cleaned_lines = Vec::new();

    for line in contents.lines() {
        let trimmed = line.trim();

        if !trimmed.is_empty() && seen.insert(trimmed.to_string()) {
            cleaned_lines.push(trimmed.to_lowercase());
        }
    }

    println!("Cleaned Output:\n");
    for (i, line) in cleaned_lines.iter().enumerate() {
        println!("{}: {}", i + 1, line);
    }
}

