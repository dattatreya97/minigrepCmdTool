
use std::error::Error;
use std::fs;
use std::env;
use walkdir::WalkDir;
use std::fs::metadata;


pub enum SearchType{
    File,
    Directory,
}

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

        println!("Searching for a file");

        if let Err(e) = search_only_file(config){
            eprintln!("{}",e);
        }
    }else{
        println!("Searching in a directory");

        if let Err(e)=search_directory(&config){
            eprintln!("Directory search failure , {}",e);
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

pub fn search_directory(config:&Config) ->Result<(),Box<dyn Error>>{
    
    let path = config.filename.clone();
    //let paths = fs::read_dir(congif.filename).unwrap();
    for entry in WalkDir::new(path) {
        //let md = metadata(entry?.path()).unwrap();
        //println!("{}",entry?.path().display());

        //To-Do :: Differentiate between Dir and File using MetaData, but getting copy error. Need to check

        let contents = fs::read_to_string(entry?.path())?;
        print_results(config, &contents);
    }

    Ok(())
}

pub fn print_results(config:&Config,contents:&str){

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    
    for x in results {
        println!("{}",x);
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

        assert_eq!(vec!["Duct tape is available today.","ABdUCt him."], search_case_insesnsitive(query, contents));
    }
}
