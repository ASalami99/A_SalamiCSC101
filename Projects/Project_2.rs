fn main() {
	let toshiba:f64 = 450_000.00 * 2.00; //I multiplied the value of toshiba by 2 because the quantity is 2.
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00 * 3.00; //I multiplied the value of hp by 3 because the quantity is 3.
	let dell:f64 = 2_850_000.00 * 3.00; //I multiplied the value of dell by 3 because the quantity is 3.
	let acer:f64 = 250_000.00;
	let sum = toshiba + mac + hp + dell + acer;
    let average:f64 = sum / 10.00; //I divided the value of the sum by 5 because the total quantity is 10.
	println!("The total sum of the sales is {} and the average is {}",sum,average);
}