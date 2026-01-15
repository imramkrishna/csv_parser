use serde_json::{Value, Map};
use crate::get_csv_data::get_csv_data;
use crate::get_headers;
use crate::save_file_to_json::save_file_to_json;

pub fn parse_csv(filename: &str) {
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let first_line = contents.lines().next().unwrap();
    let headers = get_headers::get_headers(first_line.to_string());

    // Store as Vec<Map<String, Value>> instead of Vec<String>
    let mut csv_data: Vec<Map<String, Value>> = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        if index == 0 {
            continue;
        }
        let data = get_csv_data(line.to_string());

        // Build JSON object directly
        let mut json_obj = Map::new();
        for i in 0..headers.len() {
            json_obj.insert(
                headers[i].clone(),
                Value::String(data[i].clone())
            );
        }

        csv_data.push(json_obj);
    }

    // Save the Vec of Maps
    save_file_to_json(csv_data, "data.json".to_string());
}