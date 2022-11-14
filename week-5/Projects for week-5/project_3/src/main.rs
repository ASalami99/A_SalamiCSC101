use std::io;

fn main() {

    let p = "1        Poundo Yam/Edinkaiko Soup     -N3,200".to_string();
    let f = "2        Fried Rice & Chicken          -N3,000".to_string();
    let a = "3        Amala & Ewedu Soup            -N2,500".to_string();
    let e = "4        Eba & Egusi                   -N2,000".to_string();
    let w = "5        White Rice & Stew             -N2,000".to_string();

    println!("Menu for food items available");

    println!("\nCode     Menu                           Prices");
    let menu = format!("{} \n{} \n{} \n{} \n{}",p,f,a,e,w);

    println!("{}",menu);

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nPlease enter the code for what you would like to eat today.");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let food:i64 = input1.trim().parse().expect("Not a valid number");
    

    println!("How many packs would you like to buy?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:f64 = input2.trim().parse().expect("Not a valid number");

    let mut d:f64 = 0.0;  // d is the total amount.
    let g:f64;  // g is the new amount after the discount has been calculated.
    let j:f64 = 10000.0;


    if food == 1 {
        d = quantity * 3200.0;
        println!("Your order is Poundo Yam/Edinkaiko Soup and your total bill is N{}.",d);

    } else if food == 2 {
        d = quantity * 3000.0;
        println!("Your order is Fried Rice & Chicken and your total bill is N{}.",d);

    } else if food == 3 {
        d = quantity * 2500.0;
        println!("Your order is Amala & Ewedu Soup and your total bill is N{}.",d);

    } else if food == 4 {
        d = quantity * 2000.0;
        println!("Your order is Eba & Egusi and your total bill is N{}.",d);

    } else if food == 5 {
        d = quantity * 2000.0;
        println!("Your order is White Rice & Stew and your total total bill is N{}.",d);

    } else {
        println!("Please type in a code from the menu.");
    }

    if d > j {
        g = d - ((5.0/100.0) * d);
        println!("Congratulations! You have a 5% discount. Your new bill is N{}.",g);

    }



}
