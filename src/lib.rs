#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    SyntaxError(&'static str),
}

pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<(), Error> {
    for (k, v) in build_env(path)? {
        std::env::set_var(k, v);
    }
    Ok(())
}

pub fn build_env<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<(String, String)>, Error> {
    let file = std::fs::File::open(path)?;
    parse(file)
}

pub fn parse<R: std::io::Read>(file: R) -> Result<Vec<(String, String)>, Error> {
    let mut env = vec![];
    let reader = std::io::BufReader::new(file);

    use std::io::BufRead as _;
    for line in reader.lines() {
        let line = line?;
        if let Some((k, v)) = parse_line(&line)? {
            env.push((k.to_owned(), v.to_owned()));
        }
    }
    Ok(env)
}

fn parse_line(line: &str) -> Result<Option<(&str, &str)>, Error> {
    let line = line.trim_start();
    if line.is_empty() || line.starts_with('#') {
        Ok(None)
    } else {
        let xs: Vec<&str> = line.splitn(2, '=').collect();
        if xs.len() == 2 {
            let key = xs[0];
            let val = xs[1];
            if key.find(char::is_whitespace).is_some() {
                Err(Error::SyntaxError("key cannot contain whitespaces"))
            } else {
                Ok(Some((key, val)))
            }
        } else {
            Err(Error::SyntaxError("key and value must be separated by `=`"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_denv_file() {
        let result = parse("x=123\ny=345\n".as_bytes());
        assert!(result.is_ok());
        assert_eq!(
            vec![
                ("x".to_owned(), "123".to_owned()),
                ("y".to_owned(), "345".to_owned())
            ],
            result.unwrap()
        );
    }

    #[test]
    fn ignore_comment() {
        let result = parse("x=123\n  # comment\ny=345\n".as_bytes());
        assert!(result.is_ok());
        assert_eq!(
            vec![
                ("x".to_owned(), "123".to_owned()),
                ("y".to_owned(), "345".to_owned())
            ],
            result.unwrap()
        );
    }

    #[test]
    fn ignore_empty() {
        let result = parse("x=123\n\n    \n\ny=345\n".as_bytes());
        assert!(result.is_ok());
        assert_eq!(
            vec![
                ("x".to_owned(), "123".to_owned()),
                ("y".to_owned(), "345".to_owned())
            ],
            result.unwrap()
        );
    }

    #[test]
    fn hash_in_the_middle() {
        let result = parse("x=123\ny=345# comment\n".as_bytes());
        assert!(result.is_ok());
        assert_eq!(
            vec![
                ("x".to_owned(), "123".to_owned()),
                ("y".to_owned(), "345# comment".to_owned())
            ],
            result.unwrap()
        );
    }

    #[test]
    fn equal_in_the_middle() {
        let result = parse("x=12=3\ny=345\n".as_bytes());
        assert!(result.is_ok());
        assert_eq!(
            vec![
                ("x".to_owned(), "12=3".to_owned()),
                ("y".to_owned(), "345".to_owned())
            ],
            result.unwrap()
        );
    }

    #[test]
    fn include_whitespaces() {
        let result = parse("x=a text with whitespaces\ny=345\n".as_bytes());
        assert!(result.is_ok());
        assert_eq!(
            vec![
                ("x".to_owned(), "a text with whitespaces".to_owned()),
                ("y".to_owned(), "345".to_owned())
            ],
            result.unwrap()
        );
    }

    #[test]
    fn malformed_line() {
        let result = parse("x=123\ny".as_bytes());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::SyntaxError(_)));
    }

    #[test]
    fn malformed_key() {
        let result = parse("xx x=123\ny".as_bytes());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::SyntaxError(_)));
    }
}
