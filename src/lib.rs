#[warn(dead_code)]
use std::fs;
use std::error::Error;
use::std::env;


pub struct Config{
    pub  filename:String,
    pub query:String,
    pub ignore_cases:bool
}

impl  Config {
    pub fn build(mut args:env::Args)-> Result<Config,&'static str> {
       if args.len() < 3 {
            return Err("You are such a dumbass");
        }
       args.next();
        let filename = match args.next() {
            Some(value)=>value,
            None=>return  Err("filename not found")
        }; 
        let query = match args.next() {
            Some(value)=>value,
            None=>return  Err("Query not found")
        }; 
        let is_case_sensitive = env::var("IGNORE_CASE").is_ok();
        Ok(Config {filename, query, ignore_cases:is_case_sensitive}) 
    }
}

pub fn run (config:Config)->Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    let search_result = if config.ignore_cases{
        search_case_insensitive(&content, &config.query)
    }else{
        search(&content, &config.query)
    };
    for line in  search_result{
        println!("{}",line);
    }
    Ok(())
}

pub fn search<'a>(content:&'a str,query:&str)->Vec<&'a str>{
    content
        .lines()
        .filter(|line|line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(content:&'a str,query:&str)->Vec<&'a str>{
    content
        .lines()
        .filter(|x|x.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn case_sensitive_search(){
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."],search(contents,query));
    }


    #[test]
    fn case_insensitive_search(){
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."],search(contents,query));
    }

}

