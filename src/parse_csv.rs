use std::io;
use crate::get_csv_data::get_csv_data;
use crate::get_headers;

pub fn parse_csv(filename: &str) {
    let contents=std::fs::read_to_string((filename)).expect("Something went wrong reading the file");
    let headers;
    let first_line=contents.lines().next().unwrap();
    headers=get_headers::get_headers(first_line.to_string());
    println!("{:?}",headers);
    for (index,line) in contents.lines().enumerate(){
        if index==0{
           continue; 
        }
        let data=get_csv_data(line.to_string());
        println!("{:?}",data);
    }
    
}