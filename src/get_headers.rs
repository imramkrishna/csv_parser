pub fn get_headers(line:String)->Vec<String>{
    let mut headers:Vec<String>=Vec::new();
    let headersplitted=line.split(",");
    println!("Header by using iterator");
    for header in headersplitted{
        headers.push(header.to_string());
    }
    return headers;
}