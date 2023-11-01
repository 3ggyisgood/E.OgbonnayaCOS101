fn main() {
	// items
	let to:f64 = 450_000.00;
	let ma:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00;
	let de:f64 = 2_850_000.00;
	let ac:f64 = 250_000.00;

	// quantity
	let t:f64 = 2.00;
	let m:f64 = 1.00;
	let h:f64 = 3.00;
	let d:f64 = 3.00;
	let c:f64 = 1.00;

	// total sum 
	let ts = (to * t) + (ma * m) + (de * d) + (ac * c) + (hp * h);
	println!("Total sum of sales is{}",ts);

	// total quantity
	let tq = t + m + h + d + c;
	println!("Total quantity{}",tq);

	// average
	let g = ts / tq;
	println!("Average of sales{}",g);
}