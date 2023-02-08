use std::{fs, mem, process};

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

    pub fn new(args: &mut Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("args len < 3");
        }

        let query = mem::take(&mut args[1]);
        // https://stackoverflow.com/questions/27904864/what-does-cannot-move-out-of-index-of-mean
        // Implicitly moving out of a Vec is not allowed as it would leave it in an invalid state
        // — one element is moved out, the others are not.
        // If you have a mutable Vec, you can use a method like Vec::remove to take a single value out:
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
        let mut args = vec!["1".to_string(), "2".to_string()];
        let ret = Config::new(&mut args);
        assert_eq!(ret.as_ref().unwrap().file_path, "");
        assert_eq!(ret.as_ref().unwrap().query, "2");
    }
}
