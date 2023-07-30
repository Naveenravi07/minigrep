use std::env;
use std::fs;
use::std::process;
use std::error::Error;


fn main() {
    let args : Vec<String> = env::args().collect();

    let input_data = Config::new(&args).unwrap_or_else(|err|{
        println!("problem in parsing arguments, {}",err);
        process::exit(1);
    });

    println!("filename = {} and query is : {}",input_data.filename,input_data.query);
   
    if let Err(e) = run(input_data){
        println!("Application Error, {}",e);
        process::exit(1);
    }

}

struct Config{
    filename:String,
    query:String
}

impl  Config {
    fn new(args:&[String])-> Result<Config,&str> {
        if args.len() < 3 {
            return Err("You are such a dumbass");
        }
        let filename = args[1].clone();
        let query = args[2].clone(); 
        Ok(Config {filename, query}) 
    }
}
fn run (config:Config)->Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    println!("{}",content);
    Ok(())
}
