use std::io::Read;
use std::io;

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

    if user == 1 { 

        administrator();

    }

    if user == 2 {

        p_manager();

    }

    if user == 3 {

        employee();

    }

    if user == 4 {

        customer();

    }

    if user == 5 {

        vendor();

    }

    fn administrator () {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut admin_file = String::new();
        file.read_to_string(&mut admin_file).unwrap();
        print!("{}", admin_file);
    }
    
    fn p_manager () {
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut manage_file = String::new();
        file.read_to_string(&mut manage_file).unwrap();
        print!("{}", manage_file);
    }

    fn employee () {
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut employ_file = String::new();
        file.read_to_string(&mut employ_file).unwrap();
        print!("{}", employ_file);
    }

    fn customer () {
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
        let mut custom_file = String::new();
        file.read_to_string(&mut custom_file).unwrap();
        print!("{}", custom_file);
    }

    fn vendor () {
        let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut ven_file = String::new();
        file.read_to_string(&mut ven_file).unwrap();
        print!("{}", ven_file);
    }

}
