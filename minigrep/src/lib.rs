use std::env;
use std::error::Error;
use std::fs;
use std::str::FromStr;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    let results = if config.case_sensitive {
        search(config.query.as_str(), &contents)
    } else {
        search_insensitive(config.query.as_str(), &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments");
        }
        args.next();

        let query = next_arg(&mut args, "Could not get query")?;
        let file_name = next_arg(&mut args, "Could not get file name")?;
        let case_sensitive = next_arg(&mut args, "Could not find case flag")?;
        let case_sensitive =
            bool::from_str(case_sensitive.as_str()).map_err(|_| "Failed to parse bool")?;

        Ok(Self {
            query,
            file_name,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn next_arg(args: &mut env::Args, err_msg: &'static str) -> Result<String, &'static str> {
    match args.next() {
        Some(arg) => Ok(arg),
        None => Err(err_msg),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:"], search_insensitive(query, contents));
    }
}
