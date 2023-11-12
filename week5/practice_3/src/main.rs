
use std::io;

fn main() {
    let mut imput1 = Strings::new();
    let mut imput2 = Strings::new();

    println!("Enter base: ");
    io::stdin().read_line(&mut imput1).expect("not a valid string")
     let base:f32 = imput1.trim().parse().expect("Not a valid number")

     println!("Enter height: ");
     io::stdin().read_line(&mut imput2).expect("Not a valid sting")
     let height:f32 = imput2.trim()parse().expect("Not a valid number")

     if base > 0.0{
        let area:f32 =(base * height) / 2.0
        println!("Area of a triangle: {:?}",area );
     }

     }
