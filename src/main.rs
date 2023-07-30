use std::env;

fn main() {
    let (query,filename) = read_input();
    println!("filename = {} and query is : {}",filename,query);
}

fn read_input()->(String,String){
    let args : Vec<String> = env::args().collect();
    if args.len()<3 {
        panic!("You are such a dumbasss");
    }
     (args[1].clone(),args[2].clone())
}
