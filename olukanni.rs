use std::io::Write;
use std::io;

fn main() {
        //introductory note

        println!("\nWelcome to Ernest & Young Global Limited.");
        println!("\nWe are a multinational professional services partnership headquartered in London, England.");

        //warning note
        println!("\nThis application was designed for the use of the HR department and other departments in this company. Use of this application outside of tasks relating to the company is highly prohibited. Kindly follow the instructions displayed in order to use the program adequately.");

        //activating code functions
        println!("\nSelect '7' from your keyboard to create a text file for Ms Aigbona Juliet and Mr Akpevwe Iloka");
        println!("\nSelect '8' from your keyboard to create a text file for Mr Adamu Sagamu and Mr Gbenga Daniels");
        println!("\nSelect '9' from your keyboard to create a text file for Mr Ehis Ero and Mrs Maria Akinsola");
        let mut staff_code = String::new();
        std::io::stdin().read_line(&mut staff_code).expect("Invalid Input");
        let staff_code:i32 = staff_code.trim().parse().expect("Invalid Input");

        if staff_code == 7{
            fn code_7() {
                println!("Select the staff you would like to create a text file for.
                        \nInput '1' for Ms Aigbona Juliet or Input '2' for Mr Akpevwe Iloka");
                
                        let mut file_create = String::new();
                        std::io::stdin().read_line(&mut file_create).expect("Invalid Input");
                        let file_create:i32 = file_create.trim().parse().expect("Invalid Input");

                        if file_create == 1 {
                            let _consulting = {"Job Services:\n Analytics consulting services\n Customer experience\n Cybersecurity, strategy, risk, compliance and resilience\n Digital transformation\n Risk consulting services\n Supply chain and operations\n Technology transformation"};

                            let mut file = std::fs::File::create("aigbona_juliet.txt").expect("File creation failed");
                            file.write_all(b"Name of staff: Aigbona Juliet\n");
                            file.write_all(b"Department of staff: Consulting\n");
                            file.write_all(b"Qualification of staff: B.Sc.\n"); 
                            file.write_all(_consulting.as_bytes()).expect("write failed");
                            println!("\nData Written to file");    
                    }
                        if file_create == 2 {
                            let _assurance = {"Job Services:\n Audit services\n Climate change and sustainability services\n Financial accounting advisory services\n Forensic and integrity services\n Private client audit experience\n Accounting Link\n Assurance"};

                                let mut file = std::fs::File::create("akpevwe_iloka.txt").expect("File creation failed");
                                file.write_all(b"Name of staff: Akpevwe Iloka\n");
                                file.write_all(b"Department of staff: Assurance\n");
                                file.write_all(b"Qualification of staff: HND\n");
                                file.write_all(_assurance.as_bytes()).expect("write failed");
                                println!("\nData Written to file");
                        }
            }

            //calling function
            code_7();
        }

        if staff_code == 8 {
            fn code_8() {
                println!("Select the staff you would like to create a text file for. 
                \nInput '1' for Mr Adamu Sagamu or Input '2' for Mr Gbenga Daniels");

                let mut file_create = String::new();
                        std::io::stdin().read_line(&mut file_create).expect("Invalid Input");
                        let file_create:i32 = file_create.trim().parse().expect("Invalid Input");

                        if file_create == 1 {
                            let _tax = {"Job Services:\n Tax planning\n Tax function operations\n Tax policy and controversy\n Global trade\n Tax accounting\n Tax Compliance\n Transaction tax"};
    

                            let mut file = std::fs::File::create("adamu_sagamu.txt").expect("File creation failed");
                            file.write_all(b"Name of staff: Adamu Sagamu\n");
                            file.write_all(b"Department of staff: Tax\n");
                            file.write_all(b"Qualification of staff: B.Sc.\n");
                            file.write_all(_tax.as_bytes()).expect("write failed");
                            println!("\nData Written to file");    
                    }
                        if file_create == 2 {
                            let _peopleworkforce = {"Job Services:\n Change management and experience\n HR transformation\n Integrated workforce mobility\n Learning and development consulting\n Recognition and reward advisory\n Workforce analytics\n People and workforce"};

                                let mut file = std::fs::File::create("gbenga_daniels.txt").expect("File creation failed");
                                file.write_all(b"Name of staff: Gbenga Daneils\n");
                                file.write_all(b"Department of staff: People and Workforce\n");
                                file.write_all(b"Qualification of staff: HND\n");
                                file.write_all(_peopleworkforce.as_bytes()).expect("write failed");
                                println!("\nData Written to file");
                        }

                }

                //calling function
                code_8();
        }
        
        if staff_code == 9 {
            fn code_9() {
                println!("Select the staff you would like to create a text file for.
                \nInput '1' for Mr Ehis Ero or Input '2' for Mrs Maria Akinsola");

                let mut file_create = String::new();
                        std::io::stdin().read_line(&mut file_create).expect("Invalid Input");
                        let file_create:i32 = file_create.trim().parse().expect("Invalid Input");

                        if file_create == 1 {
                            let _strategy = {"Job Services:\n Strategy consulting\n Corporate and growth strategy\n Transaction strategy and execution\n Restructuring and turnaround strategy\n Industry strategy\n Digital business building\n Commercial strategy"};
    

                            let mut file = std::fs::File::create("ehis_ero.txt").expect("File creation failed");
                            file.write_all(b"Name of staff: Ehis Ero\n");
                            file.write_all(b"Department of staff: Strategy\n");
                            file.write_all(b"Qualification of staff: M.Sc.\n");
                            file.write_all(_strategy.as_bytes()).expect("write failed");
                            println!("\nData Written to file");    
                    }
                        if file_create == 2 {
                            let _transaction = {"Job Services:\n Corporate Finance\n Divestments and carve-outs\n Sustainability and ESG Services\n M&A Advisory\n M&A Integration\n M&A technology and tools\n M&A advanced analytics"};

                                let mut file = std::fs::File::create("maria_akinsola.txt").expect("File creation failed");
                                file.write_all(b"Name of staff: Maria Akinsola\n");
                                file.write_all(b"Department of staff: Transactions and Corporate Finance\n");
                                file.write_all(b"Qualification of staff: M.Sc.\n");
                                file.write_all(_transaction.as_bytes()).expect("write failed");
                                println!("\nData Written to file");
            }
        }

        //calling function
        code_9();
    }

    loop {
        let mut input = String::new();
        println!("Would you like to access another staff's data? (Input 'Y' for yes or 'N' for no)" );
        io::stdin().read_line(&mut input).expect("Invalid String");
        let decision = input.trim();

        if decision == "Y" {
            main();
        }
    else if decision == "N"
            { println!("Thank you for using this program");
               
               break; 
            }
    }
}