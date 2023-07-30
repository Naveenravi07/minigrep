use std::env;
use::std::process;

use minigrep;

fn main() {
    let args : Vec<String> = env::args().collect();

    let input_data = minigrep::Config::build(&args).unwrap_or_else(|err|{
        println!("problem in parsing arguments, {}",err);
        process::exit(1);
    });

    println!("filename = {} and query is : {}",input_data.filename,input_data.query);
   
    if let Err(e) = minigrep::run(input_data){
        println!("Application Error, {}",e);
        process::exit(1);
    }

}

