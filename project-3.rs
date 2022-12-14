// Rust program displaying food menu
// and food items to take price from the customer

use std::io;

fn main() {
    //welcome
    println!("\nWelcome to Sail Munch!");
    println!("\nHere is the food menu:");
    
    //display menu
    println!("Food Item                                 Price");
    println!("1 for Poundo Yam/Edinkaiko Soup                 N3200.00");
    println!("2 for Fried Rice & Chicken                      N3000.00");
    println!("3 for Amala & Ewedu Soup                        N2500.00");
    println!("4 for Eba & Eguisi Soup                         N2000.00");
    println!("5 for White Rice & Stew                         N2500.00"); 
    println!("For prices more than N10,000 , you get a 5% DISCOUNT!!");
    
    
    println!("What would you like to order? (Please input a number): ");
    let mut pp = String::new();
    io::stdin().read_line(&mut pp).expect("Item not available");
    let pp:i32 = pp.trim().parse().expect("Invalid price");

    //for multiple quantitites of a customer's price
    println!("How many of that price would you like?");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Item not available");
    let quantity:f32 = quantity.trim().parse().expect("Invalid price");

    let mut aa:f32 = 0.0;
    
    if pp == 1{
        aa += 3200.00 * quantity;
            println!("Your order is Poundo Yam/Edinkaiko Soup, it costs: {}", aa)
        }
        
    if pp == 2 {
        aa += 3000.00 * quantity;
        println!("Your order is Fried Rice & Chicken, it costs {}", aa);
        }

    if pp == 3 {
        aa += 2500.00 * quantity;
            println!("Your order is Amala & Ewedu Soup, it costs: {}", aa)
        }

    if pp == 4 {
        aa += 2000.00 * quantity;
            println!("Your order is Eba & Egusi, it costs: {}", aa)
        }

    if pp == 5 {
        aa += 2500.00 * quantity;
            println!("Your order is White Rice & Stew, it costs: {}", aa)
        }

        if aa <= 10000.00
        {
            println!("Sorry, You don't get a discount, Total price is {}", aa);
        }   
        else if aa > 10000.00
        {
            let _d:f32 = (5.0/100.0) * aa; 
            println!("You get a 5% Discount!!, Total price is {}", aa - _d);
        }

        println!("Would you like to make another order? \n(Input 1 for Yes) \n(Input 0 for No) ");

        let mut pp = String::new();
        io::stdin().read_line(&mut pp).expect("Item not available");
        let pp:i32 = pp.trim().parse().expect("Invalid price");

        let mut aa:f32 = 0.0;
        loop {

    if pp == 1 {
        //display menu
    println!("Food Item                                 Price");
    println!("1 for Poundo Yam/Edinkaiko Soup                 N3200.00");
    println!("2 for Fried Rice & Chicken                      N3000.00");
    println!("3 for Amala & Ewedu Soup                        N2500.00");
    println!("4 for Eba & Eguisi Soup                         N2000.00");
    println!("5 for White Rice & Stew                         N2500.00"); 
    println!("For prices more than N10,000 , you get a 5% DISCOUNT!!");
    

    println!("What would you like to order? (Please input a number): ");
    let mut pp = String::new();
    io::stdin().read_line(&mut pp).expect("Item not available");
    let pp:i32 = pp.trim().parse().expect("Invalid price");

    //for multiple quantitites of a customer's price
    println!("How many of that price would you like?");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Item not available");
    let quantity:f32 = quantity.trim().parse().expect("Invalid price");


    }
 
    
        if pp == 1{
            aa += 3200.00 * quantity;
                println!("Your order is Poundo Yam/Edinkaiko Soup, it costs: {}", aa)
            }
            
        else if pp == 2 {
            aa += 3000.00 * quantity;
            println!("Your order is Fried Rice & Chicken, it costs {}", aa);
            }

        else if pp == 3 {
            aa += 2500.00 * quantity;
                println!("Your order is Amala & Ewedu Soup, it costs: {}", aa)
            }

        else if pp == 4 {
            aa += 2000.00 * quantity;
                println!("Your order is Eba & Egusi, it costs: {}", aa)
            }

        else if pp == 5 {
            aa += 2500.00 * quantity;
                println!("Your order is White Rice & Stew, it costs: {}", aa)
            }

            if aa <= 10000.00
            {
                println!("Sorry, You don't get a discount, Total price is {}", aa);
            }   
            else if aa > 10000.00
            {
                let _d:f32 = (5.0/100.0) * aa; 
                println!("You get a 5% Discount!!, Total price is {}", aa - _d);
            }

    if pp == 0 {
                println!("Thanks For Patronizing Sail Munch!");
                }
                break;
}
}
