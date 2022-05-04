use std::collections::HashMap;

fn evil_vector(){
    let mut evil_vec: Vec<String> = Vec::new();
    evil_vec.push(String::from("abc"));

    //let x = &evil_vec[0];
    for i in 0..10{
        evil_vec.push(String::from("bce"));
    }
    //println!("{x}");
}

fn safe_vector(){
    let mut v = Vec::new();
    
    for i in 0..10{
        println!("{} {}", v.len(), v.capacity());
        v.push(i);
    }

    for number in v {
        println!("{number}");
    }
}

fn safe_unsafe_get(){
    let mut v = Vec::new();
    for i in 0..10{
        v.push(i);
    }
    
    //safe
    match v.get(4) {
        Some(value) => println!("{value}"),
        None => println!("ERROR"),
    }
    match v.get(10) {
        Some(value) => println!("{value}"),
        None => println!("ERROR"),
    }

    //unsafe
    println!("{}", v[4]);
    println!("{}", v[10]);
}

fn hashmap_example(){
    let mut map = HashMap::new();
    map.insert("ROT", 120);
    map.insert("BLAU", 140);

    if map.contains_key("ROT"){
        if let Some(points) = map.get_mut(&"ROT"){
            println!("{points}");
            *points += 10;
            println!("{points}");
        }
    }

    for (key, value) in map{
        println!("{key} {value}");
    }

    
}

fn string_example(){
    let mut str1 = String::from("Hello, ");
    str1.push_str("World");
    println!("{str1}");
    str1.push('!');
    println!("{str1}");

    let i = 5;
    let nice = format!("Yöu have {i} points");
    println!("{nice}");
    
    
    let x = nice.get(0..3);
    if let Some(slice) = nice.get(0..2){
        println!("{slice}");
    }
    if let Some(slice) = nice.get(0..3){
        println!("{slice}");
    }
    
}

fn main() {
    
string_example();
//    hashmap_example();
//    safe_unsafe_get();
//    safe_vector();
//    evil_vector();
    
}
