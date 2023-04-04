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

    pub fn set_ignore_case(&mut self) {
        self.word = self.word.to_lowercase();
        self.file = self.file.to_lowercase();
    }

    pub fn search(&self) -> impl Iterator<Item = &str> {
        return self.file.lines().filter(|line| line.contains(&self.word));
    }
}

impl Default for Client {
    fn default() -> Client {
        Client {
            word: "".to_string(),
            file: "".to_string(),
        }
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
            ..Default::default()
        };
        assert_eq!(client.search().collect::<Vec<&str>>(), vec!["key"]);

        let client = Client {
            word: "key".to_string(),
            file: "key\nhaha\nkey2".to_string(),
            ..Default::default()
        };
        assert_eq!(client.search().collect::<Vec<&str>>(), vec!["key", "key2"]);
    }

    #[test]
    fn client_search_ignore_case() {
        let mut client = Client {
            word: "Key".to_string(),
            file: "keY\nhaha".to_string(),
            ..Default::default()
        };
        client.set_ignore_case();
        assert_eq!(client.search().collect::<Vec<&str>>(), vec!["key"]);
    }
}
