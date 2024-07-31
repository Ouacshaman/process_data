use std::*;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Test: {}", args[3]);
    let read = read_file(args[3].clone());
    println!("{}", read);
    let to_string = String::from(read);
    println!("{:#?}", process_data(&to_string));
}

#[allow(dead_code)]
fn read_file(_file_path: String) -> String{
    let path = path::Path::new(&_file_path);
    let output = fs::read_to_string(path);
    output.unwrap()  
}

#[allow(dead_code)]
fn process_data(data: &str) -> Vec<HashMap<&str,usize>> {
    let mut main_vector = Vec::new();
    let split = &data.split('\n');
    let block: &Vec<&str> = &split.clone().collect();
    for i in 0..block.len(){
        let sentence: &Vec<&str> = &block[i].split(',').collect();
        let mut map = HashMap::new();
        for j in 0..sentence.len(){
            map.entry(sentence[j]).and_modify(|v| * v += 1)
            .or_insert(1);
        }
        main_vector.push(map)
    }
    main_vector
}
