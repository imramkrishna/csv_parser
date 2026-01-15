mod parse_csv;
mod get_headers;
mod get_csv_data;
mod convert_to_json;
mod save_file_to_json;

fn main() {
    parse_csv::parse_csv("users.csv");
}
