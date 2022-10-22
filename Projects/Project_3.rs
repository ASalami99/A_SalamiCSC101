fn main() {
	let p = 210_000;
	let r = 5;
	let n = 3;
	
	let amount = p * (1 - (r / 100)) ^ n;
	let compound_interest = amount - p;  
	println!("The value of the TV set after 3 years is {} ",amount);
}