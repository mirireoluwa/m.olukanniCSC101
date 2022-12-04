fn main() {
    // Vector containing names
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Adekunle", "Mathias", "April", "Ifeoluwa"];

    // Vector containing ages
    let age = vec![16,17,19,22,21,24,18,23];

    print!("\nAge allocation:\n");

    // loop to iterate elements in the vector
    for i in 0..age.len()
    {
        // iterating through i on the vector
        print!("{} is {} years old\n", name[i], age[i]); 
    }
}
