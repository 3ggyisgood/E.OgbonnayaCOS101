use std::io::Write;
use std::io;

fn main() {

    let mut name : Vec<String> = Vec::new();
    let mut matricnumber : Vec<String> = Vec::new();
    let mut department : Vec<String> = Vec::new();
    let mut level : Vec<String> = Vec::new();

    let mut input5 = String::new();
    println!("How many students will input their information:");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let _student_personal_detail:i32 = input5.trim().parse().expect("Not a valid number");

    for val in 0.._student_personal_detail {

        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();

        println!("Enter Your Full Name: ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");

        println!("Enter Your Matriculation Number: ");
        io::stdin().read_line(&mut input2).expect("Not a valid string");

        println!("Enter Your Department: ");
        io::stdin().read_line(&mut input3).expect("Not a valid string");

        println!("Enter Your Level: ");
        io::stdin().read_line(&mut input4).expect("Not a valid string");
        let _level:i64 = input4.trim().parse().expect("Not a valid number");

        name.push(input1.to_string());
        matricnumber.push(input2.to_string());
        department.push(input3.to_string());
        level.push(input4.to_string());

    }

    let name : String = name.join("");
    let matricnumber : String = matricnumber.join("");
    let department : String = department.join("");
    let level : String = level.join("");

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("This is your personal information.".as_bytes()).expect("write failed");
    file.write_all("\nNAME: ".as_bytes()).expect("write failed");
    file.write_all(name.as_bytes()).expect("write failed");
    file.write_all("\nMATRIC NUMBER: ".as_bytes()).expect("write failed");
    file.write_all(matricnumber.as_bytes()).expect("write failed");
    file.write_all("\nDEPARTMENT: ".as_bytes()).expect("write failed");
    file.write_all(department.as_bytes()).expect("write failed");
    file.write_all("\nLEVEL: ".as_bytes()).expect("write failed");
    file.write_all(level.as_bytes()).expect("write failed");
    println!("\nData written to file.");
}