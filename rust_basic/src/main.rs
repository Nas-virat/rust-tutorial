#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(while_true)]
#![allow(unused_labels)]
#![allow(unreachable_code)]

use std::collections::HashMap;

fn main() {

    //Variables

    let a: i32 = 10; // declare variables can not change
    let b: i64;
    b = 10;
    let mut z: i32 = 3;// declare mut can change value 

    let x = 10; // declare without data type

    let (x,y) = (10,20);

    const PI: f64 = 3.14; //const

    // Tuple
    let x = (1, 3.14, 1000);
    let x: (u8, f64, i32) = (1, 3.14, 1000);
    let a = x.0;
    let b = x.1;
    let c = x.2;

    // Array
    let x:[i32;5]; // Array size 5 with data type int 32
    let x = [1,2,3,4,5];
    let x = [0;5]; //[0,0,0,0,0]

    // If
    let score = 50;
    let mut grade = "";
    
    if score >= 80{
        grade = "A";
    } 
    else if score >= 70{
        grade = "B";
    }
    else if score >= 60{
        grade = "C";
    }
    else if score >= 50{
        grade = "D";
    }
    else{
        grade = "F";
    }



    let grade = if score >= 80 {
        "A"
    } else if score >= 70 {
        "B"
    } else if score >= 60{
        "C"
    } else if score >= 50{
        "D"
    } else{
        "F"
    };

    let result = if score >= 50 {"Pass"} else {"Fail"};

    //Loop
    while true{
        println!("Hello");
        break;
    }

    'label1:loop{
        'label2:loop{
            continue 'label2;
            //break 'label1;
        }
    }

    //for 
    
    for i in 0..10{ // 0-9
        println!("{}",i);
    }

    for i in 0..=10{ // 0-10
        println!("{}",i);
    }

    // loop array
    let number = [1,2,3];
    for n in number.iter(){
        println!("{}",n);
    }

    // loop array with index
    for (i,n) in number.iter().enumerate(){
        println!("{} {}",i,n);
    }

    for n in [1,2,3].iter(){
        println!("{}",n);
    }

    //loop array of tuple

    let number = [(1,2),(3,4),(5,6)];
    for (i,j) in number.iter(){
        println!("{} {}",i,j);
    }

    //String
    let x = "Hello"; // Borrow String slice (store in stack)
    let x = String::from("Hello"); // String
    let x = "Hello".to_string(); 

    //Collection
    let mut x: Vec<i32> = Vec::new(); // Vec is growable array
    
    x.push(10);
    x.push(20);
    x.push(30);
    let y = x.pop();

    let mut x = vec![1,2,3]; // vec! macro

    //HashMap
    let mut x: HashMap<&str,&str> = HashMap::new();

    x.insert("name","John");
    x.insert("age","20");
    let y = x.get("name");
    println!("{}",y.unwrap());

    //struct
    let p = Person{
        name: String::from("John"),
        age: 20,
    };
    

}


fn get_number() -> i32{
    let a = 10;
    let b = 20;

    a + b // no need return with no ;
}

//Struct

struct Person{
    name: String,
    age: i32,
}
