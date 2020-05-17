//variables hold primitive data or references to data
//variables are immutable by default
//Rust is a block-scoped language

pub fn run() {

    //immutable variable (aka const)
    let name = "Brad";

    //mutable variable.
    let mut age = 37;

    age = 38;

    println!("My name is {} and I am {}", name, age);

    //Define a constant (const are all uppercase and have to define type (i32 = integer 32))
    const ID: i32 = 001;

    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);

}