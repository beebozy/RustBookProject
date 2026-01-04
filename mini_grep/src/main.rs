use std::env::args;
use std::fs;
use std::error::Error;
use mini_grep::search;
fn main() {

    let args:Vec<String>=env::args().collect();
   
    let config=Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    //let (query,file_path)=parse_config(&args);

    
     if let Err(e) = run(config){
        println!("Application error: {e}");
        std::process::exit(1);
     }
  //  dbg!(args);
}

struct Config{
    query:String,
    file_path:String,
}


impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
}
}

fn run(config:Config)->Result<(), Box<dyn Error>>{
  let contents =fs::read_to_string(config.file_path)?;

  for line in search(&config.query,&contents){
    println!("{line}");
  }
    
              Ok(())
    
}