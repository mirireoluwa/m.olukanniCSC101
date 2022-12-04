fn main() {
    
    // using Vec::new()
    let v : Vec<i64> = Vec::new();

    // printing the size of the vector
    println!("\nLength of Vec::new is: {}", v.len());

    // using macro
    let v = vec!["Grace", "Hakeem", "Basit", "Dolapo", "Selah"];

    //printing the size of the vector
    println!("\nLength of the vec macro is: { }", v.len());

}