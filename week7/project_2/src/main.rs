use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();
    let mut input10 = String::new();
    let mut input11 = String::new();
    let mut input12 = String::new();
    let mut input13 = String::new();

    //empty array
    let mut arr:Vec<String> = Vec::new();
    println!("Hey there!!");
    println!("How many siblings do you have? ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _number_of_siblings:i32 = input1.trim().parse().expect("Not a valid number");
    //pushing element into array
    arr.push(input1.to_string());

    for i in 0.._number_of_siblings {

    println!("Enter First Name: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    //pushing element into array
    arr.push(input2.to_string());

    println!("Enter Age: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let _age:f64 = input3.trim().parse().expect("Not a valid number");
    //pushing element into array
    arr.push(input3.to_string());

    if _age > 18.0 {
        println!("Is He/She Married or Single?: ");
        println!("Type 0 for Married and 1 for Single");
        io::stdin().read_line(&mut input4).expect("Not a valid string");
        let _mors:f64 = input4.trim().parse().expect("Not a valid number");
        //pushing element into array
        arr.push(input4.to_string());

        if _mors == 1.0 {
            println!("Is He/She a Worker or Student?: ");
            println!("Type 1 for Worker and 0 for Student");
            io::stdin().read_line(&mut input5).expect("Not a valid string");
            let _workerorstudent:f64 = input5.trim().parse().expect("Not a valid number");
            //pushing element into array
            arr.push(input5.to_string());

        if _workerorstudent == 0.0 {
            println!("Enter Name of University: ");
            io::stdin().read_line(&mut input6).expect("Not a valid string");        
            //pushing element into array
            arr.push(input6.to_string());

            println!("Enter Course of Study: ");
            io::stdin().read_line(&mut input7).expect("Not a valid string"); 
            //pushing element into array
            arr.push(input7.to_string());
            
        }
        }
        else if _mors == 0.0{
            println!("Does He/She have Children in their marriage?: ");
            println!("Type 1 for Yes and 0 for No");
            io::stdin().read_line(&mut input8).expect("Not a valid string");
            let _childreninmarriage:f64 = input8.trim().parse().expect("Not a valid number");
            //pushing element into array
            arr.push(input8.to_string());
            
            println!("What is the  name of the city that His/Her family lives in?: ");
            io::stdin().read_line(&mut input9).expect("Not a valid string");
            //pushing element into array
            arr.push(input9.to_string());
        }
    
    }
    else if _age < 18.0 {
        println!("Has He/She wrritten WAEC?: ");
        println!("Type 1 for Yes and 0 for No");
        io::stdin().read_line(&mut input10).expect("Not a valid string");
        let _wwaec:f64 = input10.trim().parse().expect("Not a valid number");
        //pushing element into array
        arr.push(input10.to_string());
        
        if _wwaec == 0.0{
    
        println!("Enter name of Secondary School: ");
        io::stdin().read_line(&mut input11).expect("Not a valid string");
        //pushing element into array
        arr.push(input11.to_string());

        println!("Enter Current Class Level: ");
        io::stdin().read_line(&mut input12).expect("Not a valid string");
        //pushing element into array
        arr.push(input12.to_string());
    
        }
        if _wwaec == 1.0 {
            println!("Sorry but your data is not defined in the database. Please input your status");
            io::stdin().read_line(&mut input13).expect("Not a valid string");
            //pushing element into array
            arr.push(input13.to_string());

        }
    }
    for val in arr.iter(){
        println!("{:?}",val);
        }
    }
}