use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query : String,
    pub file_name : String,
    pub case_sensitive : bool,
}

impl Config {

    pub fn new (args : &[String]) -> Result<Config , &'static str> {

        if args.len() < 3 { 
            return Err("il n'y a pas assez d'arguments");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();
        
        let case_sensitive = env::var("MINIGREP_INSENSITIVE_CASE").is_err();
        
        Ok(Config { 
            query , 
            file_name ,
            case_sensitive ,
        } )

    }


}

pub fn run(config : Config) -> Result<() , Box<dyn Error>> { 

    let content = fs::read_to_string(config.file_name)?;

    let results = if config.case_sensitive {
       
        println!("using sensitive case");
        search(&config.query , &content)
    }else {
        println!("using insensitve case");
        search_case_insensitive(&config.query , &content)
    };


    for line in results {
        println!("{}" , line);
    }

    Ok(())
}


pub fn search<'a>(query : &str , content : &'a str) -> Vec<&'a str> {

    let mut result : Vec<&'a str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}


pub fn search_case_insensitive<'a>(query : &str ,content:&'a str ) -> Vec<&'a str>{

    let query = query.to_lowercase();

    let mut result  = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query){
            result.push(line);

        }
    }
    result
}

#[cfg(test)]
mod test  {
    use super::*;

    #[test]
    fn case_sensitive(){
        
        let query = "duct";
        let content = "\
        Rust : 
safety , speed , productivity.
Get all three at the same time.
Duck tape.";

        assert_eq!(
            vec!["safety , speed , productivity."],
            search(query , content)
        );

    }



    #[test]
    fn case_insensitive(){

        let query = "rUst";
        let centent = "\
Rust : 
safety, speed, productivity.
Get all three at the same time.
It's not rustic.";

        assert_eq!(
            vec!["Rust:" , "It's not rustic."],
            search_case_insensitive(query , content)
        );
    }
 }
