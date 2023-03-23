use std::{*, error::Error};
pub fn run(config: Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    let contents = match fs::read_to_string(config.file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}", e);
            "".to_string()
        },
    };
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    // println!("With text:\n{contents}");
    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("ignore_case").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}
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
        assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            search_case_insensitive(query, contents),
            vec!["Rust:", "Trust me."]
        );
    }

    // #[test]
    // fn file_not_found() {
    //     let config = Config {
    //         query: String::from("duct"),
    //         file_path: String::from("nonexistent.txt"),
    //         ignore_case: false,
    //     };
    //     assert!(run(config).is_err());
    // }

    #[test]
    fn missing_query() {
        let args = vec![String::from("program_name"), String::from("filename.txt")];
        assert!(Config::new(args.into_iter()).is_err());
    }

    // #[test]
    // fn missing_file_path() {
    //     let args = vec![String::from("program_name"), String::from("query")];
    //     assert!(Config::new(args.into_iter()).is_err());
    // }
}
