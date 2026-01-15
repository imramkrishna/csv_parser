use serde_json::{Value,Map};
pub fn convert_to_json(headers:Vec<String>,data:Vec<String>)->String {
    let mut json_obj=Map::new();
    for i in 0..headers.len(){
        json_obj.insert(
            headers[i].clone(),
            Value::String(data[i].clone())
        );
    }
    let final_json=serde_json::to_string_pretty(&json_obj).unwrap();
    return final_json;
}
