
//CRATES
extern crate term;

use std::path::Path;
use std::error::Error;
use std::fs;
use std::env;
use walkdir::WalkDir;




//ENUM
pub enum SearchType{
    File,
    Directory,
}

//STRUCT
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive:bool,
    pub search:SearchType,
}



impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let search_type = args[1].clone();
        let query = args[2].clone();
        let filename = args[3].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let search;

        if search_type == "file"{
            search = SearchType::File;
        }else if search_type == "dir"{
            search = SearchType::Directory;
        }else{
            println!("{}",search_type);
            panic!("Enter valid seach type, please");
        }

        Ok(Config { query, filename ,case_sensitive,search})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    

    if let SearchType::File = config.search {

        println!("Searching in a file");

        if let Err(e) = search_only_file(config){
            eprintln!("{}",e);
        }
    }else{

        println!("Searching in a directory");

        if let Err(e)=search_directory(&config){
            eprintln!("Directory search failure , {} ,{}",e,config.filename);
        }

    }
    Ok(())
}

/*
    Function to search a given string within a file
*/

pub fn search_only_file(config:Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for x in results {
        println!("{}",x);
    }

    Ok(())
}

/*
    Function to search a directory for the given text
*/

pub fn search_directory(config:&Config) ->Result<(),Box<dyn Error>>{

    for entry in WalkDir::new(&config.filename) {
        match safe_read(entry?.path()) {
            Ok(contents)=>{
                print_results(config, &contents);
            },
            Err(_e) => {
                /*
                This indicates some error in reading the path ""entry" as a FILE.
                */
            }
        }
    }

    Ok(())
}

/*
    Utility to read contents fo the file and returning error codes,if any
*/

fn safe_read<P: AsRef<Path>>(file: P)  -> Result<String, std::io::Error> {
    let mut _data = fs::read_to_string(file)?;
    Ok(_data)
}


/*
    Utility function to print lines in files which have the required input
 */
pub fn print_results(config:&Config,contents:&str){

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    let mut found:bool = false;
    for x in results {
        println!("{}",x);
        found=true;
    }

    if found {
        println!("File :: {} \n",config.filename);
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_case_insesnsitive() {
        let query = "duct";
        let contents = "\
Rust:
Duct tape is available today.
ABdUCt him.";

        assert_eq!(vec!["Duct tape is available today.","ABdUCt him."], search_case_insensitive(query, contents));
    }
}
