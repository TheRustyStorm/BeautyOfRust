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
        if let Some(points) = map.get_mut("ROT"){
            println!("{points}");
            *points += 10;
            println!("{points}");
        }

        match map.get_mut("ROT"){
            Some(points) => {
                println!("{points}");
                *points += 10;
                println!("{points}");
            },
            None => {}
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

fn skipstuff(){
    let mut x = "abcdöefgh";

    println!("{}", x.chars().nth(4).unwrap());
    for zeichen in x.chars(){
        println!("{zeichen}");
    }
}

fn borrow_vec(){
    let v = vec![1,2,3,4,5];
    let even_numbers: Vec<_> = v.iter()
        .filter(|abc| *abc % 2 == 0)
        .map(|x| x+2)
        .collect();
    println!("{:?}",even_numbers);

}

fn exercise(){

}

fn main() {
    borrow_vec();
//    skipstuff();
//    vecborrow();
//    string_example();
//    hashmap_example();
//    safe_unsafe_get();
//    safe_vector();
//    evil_vector();
    
}
