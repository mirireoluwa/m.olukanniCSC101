use std::io::Read;

fn main(){
    //introductory note

    println!("Welcome to Globacom Limited!");
    println!("We are a telecommunications company that provides integrated telecommunication services and more.");
    println!("Check out our website https://www.gloworld.com ");


    //activating code functions
    println!("User, state your role");
    println!("Select '1' for Administrator");
    println!("Select '2' for Project Manager");
    println!("Select '3' for Employee");
    println!("Select '4' for Customer");
    println!("Select '5' for Vendor");

    let mut user= String::new();
        std::io::stdin().read_line(&mut user).expect("Invalid Input");
        let user:i32 = user.trim().parse().expect("Invalid Input");

        if user == 1{
            fn administrator () {
                let mut file = std::fs::File::open("globacom_dbase").unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                print!("{}", contents);
            }
        
            administrator();
        }

        if user == 2{
            fn p_manager () {
                let mut file = std::fs::File::open("project_tb.sql").unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                print!("{}", contents);
            }

            p_manager();
        }

        if user == 3{
            fn employee () {
                let mut file = std::fs::File::open("staff_tb.sql").unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                print!("{}", contents);
            }

            employee();
        }

        if user == 4{
            fn customer () {
                let mut file = std::fs::File::open("customer_tb.sql").unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                print!("{}", contents);
            }

            customer();
        }

        if user == 5{
            fn vendor () {
                let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                print!("{}", contents);
            }

            vendor();
        }
}
