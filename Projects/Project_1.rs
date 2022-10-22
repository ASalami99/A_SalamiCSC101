fn main() {
	let p = 520_000_000;
	let r = 10;
	let n = 5;
	
	let amount = p * ((1+(r/100))^n);
	let compound_interest = amount - p;  
	println!("The Amount is {} so therefore, the Compound Interest is {}",amount,compound_interest);
}