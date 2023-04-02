#[derive(Debug)]
pub struct Client {
    word: String,
    file: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("you should give exactly 2 args")]
    InvalidArgs,
    #[error("std::io::Error: {0}")]
    IOError(#[from] std::io::Error),
}

impl Client {
    pub fn new(args: &Vec<String>) -> Result<Client, Error> {
        if args.len() != 2 {
            return Err(Error::InvalidArgs);
        }
        let file = std::fs::read_to_string(&args[1])?;
        return Ok(Client {
            word: String::from(&args[0]),
            file,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_client_invalid_args() {
        let args1: Vec<String> = vec![String::from("arg1")];
        let args2: Vec<String> = vec![
            String::from("arg1"),
            String::from("arg2"),
            String::from("arg3"),
        ];

        for args in &vec![args1, args2] {
            let res = Client::new(args);
            assert!(matches!(res.unwrap_err(), Error::InvalidArgs))
        }
    }

    #[test]
    fn new_client_invalid_filepath() {
        let args: Vec<String> = vec![
            String::from("arg1"),
            String::from("some path that must not exists"),
        ];
        let err = Client::new(&args).unwrap_err();
        assert!(matches!(err, Error::IOError(_)));
    }
}
