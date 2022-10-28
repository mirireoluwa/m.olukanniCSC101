fn main() {
	let toshiba:f64 = 450_000.00 * 2;
	let mac:f64 = 1_500_000.00 * 1;
	let hp:f64 = 750_000.00 * 3;
	let dell:f64 = 2_850_000.00 * 3;
	let acer:f64 = 250_000.00 * 1;
	
	//sum
	let sum = toshiba + mac + hp + dell + acer;
	println!("Sum is {}",  sum);
	let average = sum/10.0;
	println!("Average is {}", average);
	
}
