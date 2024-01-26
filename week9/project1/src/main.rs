use std::io::Write;

fn main() {

    let lager = ["\tLAGER", "\n1) 33 Export", "\n2) Desperados", "\n3) Goldberg", "\n4) Gulder", "\n5) Heineken", "\n6) Star\n"];

    let stout = ["\tSTOUT","\tLegend", "\tTurbo King", "\tWilliams", "\t", "\t", "\t"];
    
    let non_alcoholic = ["\t\tNon-Alcoholic",  "\t        Amstel Maltina", "\tMalta Gold", "\tFayrouz", "\t", "\t"];

    let mut file = std::fs::File::create("Nigerian Brewery Limited High Quality.txt").expect("create failed");

    let text = "This are the type of drinks we offer\n";

    file.write_all(text.as_bytes()).expect("write failed");

    for x in 0..6{
    file.write_all(lager[x].as_bytes()).expect("write failed");

    file.write_all(stout[x].as_bytes()).expect("write failed");

    file.write_all(non_alcoholic[x].as_bytes()).expect("Failed to write");
    }

    println!("\nData written to file.");
    
}
