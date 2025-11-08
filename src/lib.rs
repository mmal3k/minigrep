use std::fs;
use std::error::Error;


pub struct Config {
    pub search : String,
    pub file_name : String,
}

impl Config {

    pub fn new (args : &[String]) -> Result<Config , &'static str> {

        if args.len() < 3 { 
            return Err("il n'y a pas assez d'arguments");
        }

        let search = args[1].clone();
        let file_name = args[2].clone();
        
        
        Ok( Config { search , file_name } )
    }


}

pub fn run(config : Config) -> Result<() , Box<dyn Error>> { 

    let content = fs::read_to_string(config.file_name)?;


    println!("the text :\n{}" , content);
    Ok(())
}

