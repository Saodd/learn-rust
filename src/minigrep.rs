#[derive(Debug)]
pub struct Client {
    word: String,
    file: String,
}

impl Client {
    pub fn new(keyword: &str, filepath: &str) -> Result<Client, std::io::Error> {
        return Ok(Client {
            word: String::from(keyword),
            file: std::fs::read_to_string(filepath)?,
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
    fn new_client_invalid_filepath() {
        Client::new("keyword", "some path that must not exists").expect_err("should return error");
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
