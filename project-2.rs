// Rust program to find the incentives of employees dependent on their age

use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\n Input your age:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f32 = input1.trim().parse().expect("Not a valid number");

    println!("\n Input your number of years of experience:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience:f32 = input2.trim().parse().expect("Not a valid number");

    if age <= 60.0 && experience >= 5.0 && experience <= 42.0
    {
        println!("Your incentive is N1,560,000 annually");
    }
    else if age > 60.0 && experience >= 5.0 && experience <= 42.0
    {
        println!("You are should be retired");
    }
    else if age > 60.0 && experience >= 5.0 && experience > 42.0
    {
        println!("You should be retired");
    }
    else if age >= 40.0 && age <=59.0 && experience >= 5.0 && experience <21.0
    {
        println!("Your incentive is N1,560,000 annually");
    }
    else if age >= 40.0 && experience <= 5.0
    {
        println!("Your incentive is N100,000 annually");
    }
    else if age >= 40.0 && experience >= 21.0
    {
        println!("This is impossible, you have no incentive here");
    }
    else if age >= 30.0 && age < 40.0 && experience >= 5.0 && experience <= 12.0
    {
        println!("Your incentive is N1,480,000 annually");
    }
    else if age >= 30.0 && age < 40.0 && experience <= 5.0
    {
        println!("Your incentive is N100,000 annually");
    }
    else if age >= 30.0 && age < 40.0 && experience >= 12.0
    {
        println!("This is impossible, you have no incentive here");
    }
    else if age < 28.0 && experience >= 5.0 && experience <= 9.0
    {
        println!("Your incentive is N1,300,000 annually");
    }
    else if age < 28.0 && experience <= 5.0
    {
        println!("Your incentive is N100,000 annually");
    }
    else if age < 28.0 && experience >= 9.0
    {
        println!("This is impossible, you have no incentive here");
    }
    else if age <= 18.0 && experience >= 5.0
    {
        println!("You have no job here");
    }
    else if age <= 18.0 && experience <= 5.0
    {
        println!("You have no job here");
    }
    else
    {
        println!("Your incentive is N100,000 per month");
    }
}
