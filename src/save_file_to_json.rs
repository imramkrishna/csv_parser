use std::fs::OpenOptions;
use std::io::Write;
pub fn save_file_to_json(data: String, filename: String) {
    let mut file = OpenOptions::new()
        .create(true) // Create file if it doesn't exist
        .append(true) // Append mode (don't overwrite)
        .open(&filename)
        .expect("Unable to open file");

    writeln!(file, "{}", data).expect("Unable to write to file");

    println!("✅ Appended to: {}", filename);
}
