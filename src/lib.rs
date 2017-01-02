extern crate exec;

use std::error::Error;
use std::io::BufRead;

pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<(), String> {
    for (k, v) in try!(build_env(path)) {
        std::env::set_var(k, v);
    }
    return Ok(());
}

pub fn build_env<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<(String, String)>, String> {
    match std::fs::File::open(path) {
        Ok(f) => parse(f),
        Err(e) => Err(e.description().to_owned()),
    }
}

pub fn parse<R: std::io::Read>(file: R) -> Result<Vec<(String, String)>, String> {
    let mut env = vec![];
    let reader = std::io::BufReader::new(file);
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                if let Some((k, v)) = try!(parse_line(&line)) {
                    env.push((k.to_owned(), v.to_owned()));
                }
            }
            Err(e) => {
                return Err(e.description().to_owned());
            }
        }
    }
    return Ok(env);
}

fn parse_line(line: &str) -> Result<Option<(&str, &str)>, &str> {
    let line = line.trim_left();
    if line.is_empty() || line.starts_with("#") {
        return Ok(None);
    } else {
        let xs: Vec<&str> = line.splitn(2, "=").collect();
        if xs.len() == 2 {
            let key = xs[0];
            let val = xs[1];
            if key.find(char::is_whitespace).is_some() {
                return Err("key cannot contain whitespaces");
            } else {
                return Ok(Some((key, val)));
            }
        } else {
            return Err("key and value must be separated by `=`");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parse_denv_file() {
        assert_eq!(Ok(vec![("x".to_owned(), "123".to_owned()),
                           ("y".to_owned(), "345".to_owned())]),
                   parse("x=123\ny=345\n".as_bytes()));
    }

    #[test]
    fn ignore_comment() {
        assert_eq!(Ok(vec![("x".to_owned(), "123".to_owned()),
                           ("y".to_owned(), "345".to_owned())]),
                   parse("x=123\n  # comment\ny=345\n".as_bytes()));
    }

    #[test]
    fn ignore_empty() {
        assert_eq!(Ok(vec![("x".to_owned(), "123".to_owned()),
                           ("y".to_owned(), "345".to_owned())]),
                   parse("x=123\n\n    \n\ny=345\n".as_bytes()));
    }

    #[test]
    fn hash_in_the_middle() {
        assert_eq!(Ok(vec![("x".to_owned(), "123".to_owned()),
                           ("y".to_owned(), "345# comment".to_owned())]),
                   parse("x=123\ny=345# comment\n".as_bytes()));
    }

    #[test]
    fn equal_in_the_middle() {
        assert_eq!(Ok(vec![("x".to_owned(), "12=3".to_owned()),
                           ("y".to_owned(), "345".to_owned())]),
                   parse("x=12=3\ny=345\n".as_bytes()));
    }

    #[test]
    fn include_whitespaces() {
        assert_eq!(Ok(vec![("x".to_owned(), "a text with whitespaces".to_owned()),
                           ("y".to_owned(), "345".to_owned())]),
                   parse("x=a text with whitespaces\ny=345\n".as_bytes()));
    }

    #[test]
    fn malformed_line() {
        assert!(parse("x=123\ny".as_bytes()).is_err());
    }

    #[test]
    fn malformed_key() {
        assert!(parse("xx x=123\ny".as_bytes()).is_err());
    }
}
