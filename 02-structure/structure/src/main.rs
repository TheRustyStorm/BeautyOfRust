
struct Adder{
    number: i32
}

impl Adder{

    fn new(number: i32) -> Adder{
        Adder{number: number}
    }

    fn add_3(&self) -> i32{
        self.number + 3
    }

    fn add_5(&self) -> i32{
        self.number + 5
    }
}


fn add_3(number: i32) -> i32{
    number + 3
}

fn sum_of_vector(vec: &Vec<i32>) -> i32{
    let mut sum = 0;
    for elem in vec{
        sum += elem;
    }
    sum
}

fn add_to_vector(vec: &mut Vec<i32>, value: i32){
    vec.push(value);
}

fn main() {

    for i in 0..200{
        match i as u8 {
            0 => println!("MAGIC"),
            1..=3 => println!("SWAG"),
            4..=255 => println!("WILL IT?"),
        }
    }

    let mut v = [2;10];
    v[1] = 5;
    println!("{}", v[1]);
    println!("{}", v[2]);

    let mut v = vec![1,2,3,4];
    add_to_vector(&mut v, 12);
    println!("{}", sum_of_vector(&v));
    println!("{}", sum_of_vector(&v));
    
    
    let adder = Adder::new(5);
    println!("{}", adder.add_3());
    println!("{}", adder.add_5());

    println!("{}",add_3(7));
/*
    let a = 5;
    let b = 3;
    if a > b{
        println!("BIG");
    } else{
        println!("SMOL");
    }

    for i in 1..20{
        match i % 5{
            0 => println!("5!"),
            1..=3 => println!("lame"),
            _ => println!("else") 
        }
    }

    let x = 5;
    let mut counter = 0;
    while counter < x {
        println!("Brrrt");
        counter += 1;
    }

    for i in 0..10{
        println!("{i}");
    }

    let v = vec![10,20,30,40,50];
    for element in v{
        println!("{element}");
    }

    loop{
      //  println!("BRRRT");
    }
    */
}
