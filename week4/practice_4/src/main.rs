fn main() {
    let fullname = "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic UNiversity";


    let mut school = "School of Science".to_string();
    // push string
    school.push_str(" and Technology");


    println!("My name is {:?}",fullname );
    // check lenght
    println!("The length my fullname is: {:?}",fullname.len() );
    println!("I am a student of {:?} department",department );
    println!("{:?}",school );
    println!("{:?}",uni );
}
