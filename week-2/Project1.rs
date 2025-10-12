fn main() {
	let principal: f64 = 520_000_000.0;
	let rate: f64 = 10.0;
	let time: f64 = 5.0;

	// formula for simple interest
	let amount = principal * (1.0 + rate / 100.0).powf(time);
	println!("Amount is {}", amount);
	let compoundinterest = amount - principal;
	println!("Compound Interest is {}", compoundinterest);
}