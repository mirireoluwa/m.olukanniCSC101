//Rust program that performs mathematical calculations

use std::io;
use std::f32;

fn main() {
    println!("\nThe following are formulas to solve the areas and volume of shapes:");
    println!("T = Area of Trapezium");
    println!("R = Area of Rhombus");
    println!("P = Area of Parallelogram");
    println!("C = Area of cube");
    println!("V = Volume of Cylinder");

    println!("Select the formula you would like to use: ");
    let mut formula = String::new();
    io::stdin().read_line(&mut formula).expect("Invalid input");

    if formula.trim() == "T" {

        fn trapezium() {
        
            let mut input1 = String::new();
            println!("Input value of height: ");
            io::stdin().read_line(&mut input1).expect("Not a valid string");
            let height: f32 = input1.trim().parse().expect("Invalid input");

            let mut input2 = String::new();
            println!("Input value of first base: ");
            io::stdin().read_line(&mut input2).expect("Not a valid string");
            let base1: f32 = input2.trim().parse().expect("Invalid input");

            let mut input3 = String::new();
            println!("Input value of second base: ");
            io::stdin().read_line(&mut input3).expect("Not a valid string");
            let base2: f32 = input3.trim().parse().expect("Invalid input");

            let areaoftrapezium:f32 = (height/2.0) * (base1 + base2);

            println!("Area of a Trapezium is {}", areaoftrapezium);
    }  
        //call add function with arguments
        trapezium();
}

else if formula.trim() == "R" {
    fn rhombus() {

            let mut input1 = String::new();
            println!("Input value of first diagonal: ");
            io::stdin().read_line(&mut input1).expect("Not a valid string");
            let diagonal1: f32 = input1.trim().parse().expect("Invalid input");

            let mut input2 = String::new();
            println!("Input value of second diagonal: ");
            io::stdin().read_line(&mut input2).expect("Not a valid string");
            let diagonal2: f32 = input2.trim().parse().expect("Invalid input");

            let areaofrhombus:f32 = (1.0/2.0) * (diagonal1 * diagonal2);

            println!("Area of a Rhombus is {}", areaofrhombus);
            
    }
        //call add function with arguments
        rhombus();
}

else if formula.trim() == "P" {
    fn parallelogram() {
        
            let mut input1 = String::new();
            println!("Input value of base: ");
            io::stdin().read_line(&mut input1).expect("Not a valid string");
            let base: f32 = input1.trim().parse().expect("Invalid input");

            let mut input2 = String::new();
            println!("Input value of altitiude: ");
            io::stdin().read_line(&mut input2).expect("Not a valid string");
            let altitude: f32 = input2.trim().parse().expect("Invalid input");

            let areaofparallelogram:f32 = base * altitude;

            println!("Area of a Parallelogram is {}", areaofparallelogram);
        }
        
        //call add function with arguments
        parallelogram();
}    

else if formula.trim() == "C" {
    fn cube() {

            let mut input1 = String::new();
            println!("Input value of length: ");
            io::stdin().read_line(&mut input1).expect("Not a valid string");
            let lengthofside: f32 = input1.trim().parse().expect("Invalid input");
            
            let areaofcube:f32 = 6.0 * (lengthofside).powf(2.0);

            println!("Area of cube is {}", areaofcube);

        }
        //call add function with arguments
        cube();
}    

else if formula.trim() == "V" {
    fn cylinder() {
    
        let r = f32::consts::PI;

        let mut input1 = String::new();
        println!("Input value of radius: ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let radius: f32 = input1.trim().parse().expect("Invalid input");

        let mut input2 = String::new();
        println!("Input value of height: ");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let height: f32 = input2.trim().parse().expect("Invalid input");

        let volumeofcylinder:f32 = r * radius.powf(2.0) * height;
        println!("Volume of cylinder is {}", volumeofcylinder);
    }

        //call add function with arguments
        cylinder();
}
}
