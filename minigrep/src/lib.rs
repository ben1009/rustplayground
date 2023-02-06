use std::{fs, process};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("args is less than 3");
        }

        let query = args.get(1).unwrap().clone();
        let file_path = args.get(2).unwrap().clone();

        let config = Config { file_path, query };
        Ok(config)
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    for l in content.lines() {
        if l.contains(query) {
            ret.push(l);
        }
    }

    return ret;
}

pub fn run_grep(config: &Config) {
    let file_content = fs::read_to_string(&config.file_path).unwrap_or_else(|e| {
        eprintln!("exit with error: {}", e);
        process::exit(1);
    });

    let ret = search(config.query.as_str(), file_content.as_str());
    println!("ret: {:?}", ret)
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn args_len_less_3() {
        let args = &vec![String::from("1"), String::from("2")];
        let ret = Config::new(args);

        assert_eq!(ret.unwrap_err(), "args is less than 3")
    }

    #[test]
    fn io_error() {
        let args = &vec![String::from("1"), String::from("2"), String::from("3")];
        let ret = Config::new(args);

        assert_eq!(ret.unwrap_err(), "io error")
    }
}
