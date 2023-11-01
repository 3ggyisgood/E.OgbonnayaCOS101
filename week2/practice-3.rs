fn main() {
	let ptv:f64 = 210_000.0;
	let rtv:f64 = 5.00;
	let ntv:f64 = 3.0;

	// depreciation
	let r = 1.0 - (rtv / 100.0);
	let n = f64:: powf(r,ntv);
	let p = ptv * n;
	println!("Depreciation is{}",p);
}