use std::io;

fn aot(a: f32, b: f32, c:f32){       //aot- Area of Trapezium
    let area:f32 = (a/2.0) * (b + c);
    println!("Area of the Trapezium is = {}",area);
    }

fn aor(a: i32, b: i32){       //aot- Area of Rhombus
    let area:i32 = (a * b) / 2;
    println!("Area of the Rhombus is = {}",area);
    }

fn aop(a: i32, b: i32){       //aot- Area of Parallelogram
    let area:i32 = a * b ;
    println!("Area of the Parallelogram is = {}",area);
    }

fn aoc(a: i32){       //aot- Area of Cube
    let area:i32 = 6 * a * a;
    println!("Area of the cube is = {}",area);
    }

fn voc(a: f32, b: f32){       //voc- Volume of Cylinder
    let pi:f32 = 3.142;
    let volume:f32 = pi * a *a * b;
    println!("Volume of the Cylinder is = {}",volume);
    }

fn main() {
    let t = "1.     Area of a Trapezium.".to_string();
    let r = "2.     Area of a Rhombus.".to_string();
    let p = "3.     Area of a Parallelogram.".to_string();
    let c = "4.     Area of a Cube.".to_string();
    let cy = "5.     Volume of a Cylinder.".to_string();
    
    println!("Calculator for finding area and volume of shapes.");
    
    println!("\nCode   Formula");
    let formula = format!("{} \n{} \n{} \n{} \n{}",t,r,p,c,cy);
    println!("{}",formula);

    println!("\nPlease select the code of the formula you would like to use.");
    let mut inputa = String::new();
    io::stdin().read_line(&mut inputa).expect("Not a vaild string");
    let code:i64 = inputa.trim().parse().expect("Not a valid number");


    if code == 1{ 

    let mut input1 = String::new();
    println!("Enter the height of the trapezium:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the first base of the trapezium:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter the second base of the trapezium:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Invalid input");

    aot(a, b, c);

} else if code == 2 {

    let mut input1 = String::new();
    println!("Enter the first diagonal of the rhombus:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the second diagonal of the rhombus:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Invalid input");

    aor(a, b);

} else if code == 3 {

    let mut input1 = String::new();
    println!("Enter the base of the parallelogram:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the altitude of the parallelogram:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Invalid input");

    aop(a, b);

} else if code == 4 {

    let mut input1 = String::new();
    println!("Enter the length of one side of the cube:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");

    aoc(a);

} else if code == 5 {

    let mut input1 = String::new();
    println!("Enter the radius of the cylinder:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the heigth of the cylinder:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Invalid input");

    voc(a, b);

}


}


