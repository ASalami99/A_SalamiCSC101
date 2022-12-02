use std::io;

fn main() {

    let mut input1 = String::new;
    let mut input2 = String::new;

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a vaild string");

    println!("How many siblings do you have?");
    io::stdin().read_line(&mut input2).expect("Not a vaild string");
}   let nos:i32 = input2.trim().parse().expect("Not a valid number");
