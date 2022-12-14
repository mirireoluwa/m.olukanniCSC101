use std::io::Write;

fn main() {
    //Lager type
    let lager = vec!["Lager", "33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];

    let stout = vec!["Stout", "Legend", "Turbo King", "Williams"];

    let n_alcoholic = vec!["Non-Alcoholic", "Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

            let mut lager_str = String::new();
            for i in 0..lager.len()
            {
            lager_str +=  &lager[i];
            lager_str += "\n";
            }
            lager_str +=  "\n\n\n";

    let mut stout_str = String::new();
    for i in 0..stout.len()
    {
        stout_str += &stout[i];
        stout_str += "\n";
    }

    stout_str +=  "\n\n\n";


    let mut n_alcoholic_str = String::new();
    for i in 0..n_alcoholic.len()
    {
        n_alcoholic_str += &n_alcoholic[i];
        n_alcoholic_str += "\n";
    }

    n_alcoholic_str +=  "\n\n\n";

   let mut file = std::fs::File::create("project-1.txt").expect("create failed");
    file.write_all("Welcome to Nigerian Brewery Limited! (Inc.1946)\n Here are some high quality drinks we have:\n"
        .as_bytes()).expect("write failed");
    file.write_all(lager_str.as_bytes()).expect("write failed");
    file.write_all(stout_str.as_bytes()).expect("write failed");
    file.write_all(n_alcoholic_str.as_bytes()).expect("write failed");
    println!("\nData Written to File." );
    
}
