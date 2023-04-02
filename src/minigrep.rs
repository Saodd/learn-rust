#[derive(Debug)]
pub struct Client {
    word: String,
    file: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("you should give exactly 3 args: executable keyword filepath")]
    InvalidArgs,
    #[error("std::io::Error: {0}")]
    IOError(#[from] std::io::Error),
}

impl Client {
    pub fn new(args: &Vec<String>) -> Result<Client, Error> {
        if args.len() != 3 {
            return Err(Error::InvalidArgs);
        }
        return Ok(Client {
            word: String::from(&args[1]),
            file: std::fs::read_to_string(&args[2])?,
        });
    }

    pub fn search(&self) -> Vec<&str> {
        let mut res: Vec<&str> = Vec::new();
        let word = &self.word;
        for line in self.file.lines() {
            if line.contains(word) {
                res.push(line);
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_client_invalid_args() {
        let args1: Vec<String> = vec![String::from("arg1")];
        let args2: Vec<String> = vec![String::from("arg1"), String::from("arg2")];

        for args in &vec![args1, args2] {
            let res = Client::new(args);
            assert!(matches!(res.unwrap_err(), Error::InvalidArgs))
        }
    }

    #[test]
    fn new_client_invalid_filepath() {
        let args: Vec<String> = vec![
            String::from("arg1"),
            String::from("arg2"),
            String::from("some path that must not exists"),
        ];
        let err = Client::new(&args).unwrap_err();
        assert!(matches!(err, Error::IOError(_)));
    }

    #[test]
    fn client_search() {
        let client = Client {
            word: "key".to_string(),
            file: "key\nhaha".to_string(),
        };
        assert_eq!(client.search(), vec!["key"]);

        let client = Client {
            word: "key".to_string(),
            file: "key\nhaha\nkey2".to_string(),
        };
        assert_eq!(client.search(), vec!["key", "key2"]);
    }
}
