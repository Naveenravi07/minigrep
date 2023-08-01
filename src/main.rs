use std::env;
use::std::process;

use minigrep;

fn main() {
    let args : Vec<String> = env::args().collect();

    let input_data = minigrep::Config::build(&args).unwrap_or_else(|err|{
        eprintln!("problem in parsing arguments, {}",err);
        process::exit(1);
    });
   
    if let Err(e) = minigrep::run(input_data){
        eprintln!("Application Error, {}",e);
        process::exit(1);
    }

}

