use serde_json::{Value,Map};
use crate::convert_to_json::convert_to_json;
use crate::get_csv_data::get_csv_data;
use crate::get_headers;
use crate::save_file_to_json::save_file_to_json;

pub fn parse_csv(filename: &str) {
    let contents=std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let headers;
    let first_line=contents.lines().next().unwrap();
    headers=get_headers::get_headers(first_line.to_string());
    let mut csv_data:Vec<String>=Vec::new();
    for (index,line) in contents.lines().enumerate(){
        if index==0{
           continue;
        }
        let data=get_csv_data(line.to_string());
        let json_data=convert_to_json(headers.clone(),data);
        save_file_to_json(json_data.clone(),"data.json".to_string());
        csv_data.push(json_data);
    }
    println!("{:?}",csv_data);
    
}