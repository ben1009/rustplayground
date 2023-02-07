use std::{fs, process};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // the book' s case
    // impl Config {
    //     pub fn build(
    //         mut args: impl Iterator<Item = String>,
    //     ) -> Result<Config, &'static str> {

    // also do clone in s.into_string(), so nothing fancy

    // #[stable(feature = "env", since = "1.0.0")]
    // impl Iterator for Args {
    //     type Item = String;
    //     fn next(&mut self) -> Option<String> {
    //         self.inner.next().map(|s| s.into_string().unwrap())
    //     }
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         self.inner.size_hint()
    //     }
    // }

    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("args len < 3");
        }

        let query = String::from(&args[1]);
        let file_path = args[2].to_string();
        let config = Config { file_path, query };
        Ok(config)
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    return content
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

pub fn run_grep(config: &Config) {
    let file_content = fs::read_to_string(&config.file_path).unwrap_or_else(|e| {
        eprintln!("exit with error: {}", e);
        process::exit(1);
    });

    let ret = search(config.query.as_str(), file_content.as_str());
    println!("ret: {:?}", ret);
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn args_len_less_3() {
        let args = vec!["1".to_string(), "2".to_string()];
        let ret = Config::new(args.as_slice());
        assert_eq!(ret.as_ref().unwrap().file_path, "");
        assert_eq!(ret.as_ref().unwrap().query, "2");
    }
}
