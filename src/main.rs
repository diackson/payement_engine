use csv::{ReaderBuilder, Trim};
//use std::env;
use std::fs::File;
use std::path::Path;
use std::process;
use payment_engine::read_transaction_file;
fn main() {
    //let args: Vec<String> = env::args().collect();
    let transaction_input = String::from("./data/transactionTest.csv");// parse_config(&args);
    let transaction_input = parse_csv_reader(&transaction_input);

    match read_transaction_file(transaction_input) {
        Ok(()) => eprintln!("Terminated"),
        Err(err) => eprintln!("An error occured during process {:#?}", err),
    };
}

// fn parse_config(args: &[String]) -> &str {
//     if args.len() != 2 {
//         eprintln!("Process need a file to execute");
//         process::exit(1);
//     }

//     &args[1]
// }

fn parse_csv_reader(file_path: &str) -> csv::Reader<File>{
    match ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(true)
        .from_path(Path::new(file_path)){
            Ok(input) => input,
            Err(err) => {
                eprintln!("Failed to open file {:#?}", err);
                process::exit(2)    
            }
        }
}