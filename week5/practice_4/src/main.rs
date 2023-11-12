
use std::io;

fn main() {
    let mut imput1 = String::new();
    let mut imput2 = String::new();


    println!("Enter your name: ");
    io::stdin().read_line(&mut imput1).expect("Not a valid string");

    println!("Enter your age");
    io::stdin().read_line(&mut imput2).expect("Not a valid string");
    let age:f32 = imput2.trim().parse().expect("Not a valid number");

    if age >= 18 {
        println!("Welcome to the party {}",imput1 );
    } else{
        println!("Oops, you are not allowed to enter the party {:?}",imput1 );
    }
}
