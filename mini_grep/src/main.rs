use std::env::args;
use std::fs;
use std::error::Error;
use mini_grep::{search,search_case_sensitive};
fn main() {

    let args:Vec<String>=env::args().collect();
   
    let config=Config::build(&args).unwrap_or_else(|err| {
         eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    //let (query,file_path)=parse_config(&args);

    
     if let Err(e) = run(config){
        eprintln!("Application error: {e}");
        std::process::exit(1);
     }
  //  dbg!(args);
}

struct Config{
    query:String,
    file_path:String,
    ignore_case:bool,
}


impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path,ignore_case })
}
}

fn run(config:Config)->Result<(), Box<dyn Error>>{
  let contents =fs::read_to_string(config.file_path)?;


  let results = if config.ignore_case {
        search_case_sensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())  
            
    
}

// --snip--

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}