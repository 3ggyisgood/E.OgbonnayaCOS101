use std::io::Write;

fn main() {
    let name_of_commisioner = ["NAME OF COMMISIONER", "\n1) Aigbogun Alamba Daudu", "\n2) Murtala Ateez Bendu", "\n3) Okorocha Calistus Ogbona", "\n4) Adewale Jimoh Akanbi", "\n5) Osazuwa Faith Etieye" ,"\n"];
    
    let ministry = ["\tMINISTRY","\n1) Internal Affairs", "\n2) Justice", "\n3) Defense", "\n4) Power & Steel", "\n5) Petroleum", "\n"];
    
    let geopolitical_zone = ["\tGEOPOLITICAL ZONE",  "\n1) South West", "\n2) North East", "\n3) South South", "\n4) South West ", "\n5) South East"];
    
    let efccname = name_of_commisioner; 
    let efccmin = ministry;
    let efccgeo = geopolitical_zone;


    let efccname : String = efccname.join("");
    let efccmin : String = efccmin.join("");
    let efccgeo: String = efccgeo.join("");


    let mut file = std::fs::File::create("EFCC CONVICTED MINISTERS.txt").expect("create failed");
    file.write_all(efccname.as_bytes()).expect("write failed");
    file.write_all(efccmin.as_bytes()).expect("write failed");
    file.write_all(efccgeo.as_bytes()).expect("write failed");
    println!("\nData written to file.");

}