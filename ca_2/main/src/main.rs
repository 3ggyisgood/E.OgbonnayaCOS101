use std::fs::File;
use std::io::Write;

struct Company{

name: String,
founded: i32,
shares: i32,
liabilities:i32,
}


impl Company{
    fn calc_lev(&self)->f32{
        (((self.shares - self.liabilities) / self.shares) * 100) as f32
    }
}

fn main() {
    let comp1 = Company{
        name:String::from("Cadbury"),
        founded:1965,
        shares: 15000000,
        liabilities: 5500000
    };
    let comp2 = Company{
        name:String::from("Champion"),
        founded: 1974,
        shares: 25000000,
        liabilities: 8000000,
    };
    let comp3 = Company{
        name:String::from("Dangote Sugar Refinery"),
        founded: 1970,
        shares: 18000000,
        liabilities: 10000000,
    };
    let comp4 = Company{
        name:String::from("Flour Mills"),
        founded: 1960,
        shares: 32000000,
        liabilities: 4000000,
    };
    let comp5 = Company{
        name:String::from("Nestle"),
        founded: 1961,
        shares: 8000000,
        liabilities: 1500000,
    };
    let comp6 = Company{
        name:String::from("Unilever"),
        founded: 1923,
        shares: 37000000,
        liabilities: 11000000,
    };
    let comp7 = Company{
        name:String::from("Honeywell"),
        founded: 1906,
        shares: 34000000,
        liabilities: 9000000,
    };
    let comp8 =  Company{
        name:String::from("Nigerian Breweries"),
        founded: 1946,
        shares: 30000000,
        liabilities: 12000000,
    };
    for Company in Company{
let mut file = 
    File::create("company_data.txt").expect("write failed");
   writeln!(Company.comp1.as_bytes()).expect("write failed");
   writeln!(Company.comp2.as_bytes()).expect("write failed");
   writeln!(Company.comp3.as_bytes()).expect("write failed");
   writeln!(Company.comp4.as_bytes()).expect("write failed");
   writeln!(Company.comp5.as_bytes()).expect("write failed");
   writeln!(Company.comp6.as_bytes()).expect("write failed");
   writeln!(Company.comp7.as_bytes()).expect("write failed");
   writeln!(Company.comp8.as_bytes()).expect("write failed");
};

  if Company.shares > 20000000.00{
        let mut lev_file = 
        File::create("lev_data.txt").expect("write Failed");
           writeln!(Company.calc_lev.as_bytes()).expect("write Failed");
    };
     if Company.liabilities < 10000000.00{
        println!("5% of leverages = {}", 0.05 * calc_lev );
    };
    }