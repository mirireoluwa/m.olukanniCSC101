// Rust program to find the roots of a quadratic equation

use std::io;

fn main() 
{
    let _a:f32 = 0.0;
    let _b:f32 = 0.0;
    let _c:f32 = 0.0;

    let _root1:f32 = 0.0;
    let _root2:f32 = 0.0;

    let _realroots:f32 = 0.0;
    let _imaginaryroots:f32 = 0.0;
    let _discriminant:f32 = 0.0;

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let discriminant:f32 = (b * b) - (4.0 * a * c);

    if discriminant > 0.0
    {
        println!("Real and distinct, they are:");
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    }
    else if discriminant < 0.0
    {
        println!("Imaginary roots, they are:");
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    }
    else if discriminant == 0.0
    {
        println!("One real root :");
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    }


}


