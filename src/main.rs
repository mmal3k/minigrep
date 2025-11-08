use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args:Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Probleme rencontre lors de l'interpretation des arguments : {}" , err);
        process::exit(1);
    });



    println!("we are looking for : {}", config.search);
    println!("file name : {}",config.file_name);


   if let Err(e) = minigrep::run(config) {
        println!("Erreur applicative : {}" , e);
        process::exit(1);
   }; 
}
