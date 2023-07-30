use std::fs;
use std::error::Error;

pub struct Config{
    pub  filename:String,
    pub query:String
}

impl  Config {
    pub fn build(args:&[String])-> Result<Config,&str> {
        if args.len() < 3 {
            return Err("You are such a dumbass");
        }
        let filename = args[1].clone();
        let query = args[2].clone(); 
        Ok(Config {filename, query}) 
    }
}
pub fn run (config:Config)->Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    println!("{}",content);
    Ok(())
}
#[cfg(test)]

mod tests{
    use super::*;
    

}

