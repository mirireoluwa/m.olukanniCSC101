fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;

	// compound interest
	let dp = 1.00 - (r/100.00);
	let a = p * dp.powf(n);
	println!("Amount is {}", a);

	// depreciation
	let d = a - p;
	println!("Depreciation is {}", d);

}