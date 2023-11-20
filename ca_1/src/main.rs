use std::io;
fn main() {
    //Assign viarbles for the diseases
    let q = "Alzheimer";
    let w = "Arrhythmia";
    let e = "Chronic Kidnew Disease";
    let r = "Diabetes";
    let t = "Arthritis";

    //Assign  viarables for the villages 
    let l = "Akpabom";
    let k = "Ngbauji";
    let j = "Atabriakang";
    let h = "Okorobilom";
    let g = "Emeremen";

    //Assign viarbles for the prices
    let a:f64 = 1200000.00;
    let b:f64 = 550000.00;
    let c:f64 = 1500000.00;
    let d:f64 = 800000.00;
    let e:f64 = 450000.00;

    //put the code to allow the user to insert their informations
  let mut name = String::new();
  let mut diagonsis = String::new();
  let mut age = String::new();
  let mut village = String::new();
  let mut siblings = String::new();
  let mut children = String::new();
  let mut response = String::new();

  //display 
  println!("WELCOME TO OTUNENE FAMILY HEALTH CENTRE");
  println!("Please enter the following information.");
  println!("Enter Name: ");
  io::stdin().read_line(&mut name).expect("Failed to read name");

  println!("Enter village: ");
  io::stdin().read_line(&mut village).expect("Failed to read village
");
  println!("Enter diagonsis: ");
  io::stdin().read_line(&mut diagonsis).expect("Failed to read diagonsis");

  println!("Enter age: ");
  io::stdin().read_line(&mut age).expect("Not a valid string");
  let age:i32 = age.trim().parse().expect("Not a valid age");

  println!("How many children do you have: ");
  io::stdin().read_line(&mut children).expect("Not a valid string");
  let children:i32 = children.trim().parse().expect("Not a valid age");

  println!("How many siblings do you have: ");
   io::stdin().read_line(&mut siblings).expect("Not a valid string");
  let siblings:i32 = siblings.trim().parse().expect("Not a valid amount");

  if diagonsis.trim() == q && village.trim() == l && age > 50 && children > 4{
    println!("Mr/Mrs {} You are eligble for a 20% discount",name );
    println!("Do you want this this discount? (y/n)");
    println!("Enter choice: ");
    io::stdin().read_line(&mut response).expect("Failed to read choice");
loop {
   if response.trim() == "n"{
   break;
    }else if response.trim() == "y"{
        break;
    }
    else {
        println!("You entered a wrong option, please use y for yes and n for no");
        io::stdin().read_line(&mut response).expect("Failed to read choice");
    }
}
    

if response.trim() == "y"{
    println!("Your price is {}",a - (a *(20.00 / 100.00)) );
}else if response.trim() == "n"{
    println!("Your price is {}",a );
}
  }
  println!("Thank you for choosing us.");
}