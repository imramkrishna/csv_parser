
use std::fs;
use serde_json::{Value, Map};

pub fn save_file_to_json(data: Vec<Map<String, Value>>, filename: String) {
    // Convert the Vec<Map> to a JSON array string
    let json_string = serde_json::to_string_pretty(&data)
        .expect("Failed to serialize JSON");

    // Write to file
    fs::write(&filename, json_string)
        .expect("Unable to write to file");

    println!("✅ Saved {} records to: {}", data.len(), filename);
}