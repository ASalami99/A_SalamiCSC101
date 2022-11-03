fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;
	
	let x = 1.0 - (r/100.00);
	let y = x.powf(n);
	let amount = p * y;
	let depreciatoin = amount - p;  
	println!("The Amount is {} therefore, the Compound Interest is {}",amount,depreciatoin);
}