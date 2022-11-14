use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a vaild string");

    println!("How long have you worked with us?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let years:i32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let age:i32 = input3.trim().parse().expect("Not a valid number");

    if years >= 20 && age >= 40 
    {
        println!("Congratulations {}, your incentive is N1,560,000",input1);
    } else if years >= 20 && age >= 30  
    {
        println!("Congratulations {}, your incentive is N1,480,000",input1);
    } else if years >= 20 && age < 28
    {
        println!("Congratulations {}, your incentive is N1,300,000",input1);
    } else 
    {
        println!("You are not experienced enough {}, your incentive is N100,000",input1);
    }

}