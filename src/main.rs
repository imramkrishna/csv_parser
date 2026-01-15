mod parse_csv;
mod get_headers;
mod get_csv_data;

fn main() {
    parse_csv::parse_csv("users.csv")
}
