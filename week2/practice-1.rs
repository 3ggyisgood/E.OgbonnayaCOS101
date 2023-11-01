 fn main() {
 	let p:f64 = 520_000_000.0;
 	let r:f64 = 10.0;
 	let n:f64 = 5.0;

 	// amount
 	let x = 1.0 + (r / 100.00);
 	let y = f64:: powf(x,n);
 	let a = p * y;
 	println!("Amount is {}",a);

 	// compund interest
 	let cin = a - p;
 	println!("Compound interest{}",cin);

 }