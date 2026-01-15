pub fn get_csv_data(line:String)->Vec<String>{
    let mut data:Vec<String>=Vec::new();
    let splitted=line.split(",");
    for split in splitted{
        data.push(split.to_string());
    }
    return data;
}