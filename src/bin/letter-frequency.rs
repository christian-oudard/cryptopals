use std::collections::HashMap;

fn main() {
    // Load a file of text, specified as a command line argument..
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let text = std::fs::read_to_string(filename).expect("Can read file");

    // Count the frequency of each byte in the text.
    let mut counter = HashMap::new();
    for c in text.bytes() {
        *counter.entry(c).or_insert(0) += 1;
    }

    // Sort the counter, putting the most common letters first.
    let mut entries: Vec<(u8, u32)> = counter.into_iter().collect();
    entries.sort_by_key(|(_, count)| *count);
    entries.reverse();

    for (byte, count) in entries.iter() {
        println!("{}: {}", *byte as char, count);
    }
}
