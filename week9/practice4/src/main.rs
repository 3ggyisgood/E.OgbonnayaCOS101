use std::fs::OpenOptions;
use std::io::Write;


fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write_all("\nHello CLass".as_bytes()).expect("write failed");
    file.write_all("\n This is the appendage to the document.".as_bytes()).expect("write failed");
        println!("File append success");
}
