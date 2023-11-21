
use crate::speaking::Speaking;

pub struct Person{
    name: String,
    age: i32,
}

impl Person{

    // constructor
    pub fn new(name: String, age: i32) -> Self{
        Person{
            name,
            age,
        }
    }

    // public method
    pub fn get_name(&self) -> &str{
        &self.name
    }

    // public method
    pub fn get_age(&self) -> i32{
        self.age
    }

    pub fn hello(&self){
        println!("Hello {} {}",self.name,self.age);
    }
}

// implement the interface
impl Speaking for Person{
    fn speak(&self){
        println!("{} is speaking",self.name);
    }
}