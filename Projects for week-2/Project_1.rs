fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.00;
	let n:f64 = 5.00;
	
	let x = 1.0 + (r/100.00); //I made the whole (1.0 + (r/100.00)) a variable x to make it easier to calculate. I also turned all the numbers to float to allow for calculations.
	let y = x.powf(n); 
	let amount = p * y;
	let compound_interest = amount - p;  
	println!("The Amount is {} therefore, the Compound Interest is {}",amount,compound_interest);
}