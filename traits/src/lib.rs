use std::collections::HashMap;
use std::fmt::Write;
use std::io::Read;

pub struct Config {
    values: Vec<(String, String)>
}

///
/// A service for managing a configuration
///
pub struct KeyValueConfigService {}

impl Config {
    pub fn new(values: HashMap<String, String>) -> Self {
        let as_vector: Vec<(String, String)> = values.into_iter().map(|(k, v)| {
            (k, v)
        }).collect();
        Config { values: as_vector }
    }
}

impl KeyValueConfigService {
    pub fn new() -> KeyValueConfigService {
        KeyValueConfigService { }
    }
}

pub trait ValueGetter {
    fn get(&self, s: &str) -> Option<String>;
}
pub trait ConfigWriter {
    fn write(&self, config: Config, to: &mut impl Write) -> std::io::Result<()>;
}
pub trait ConfigReader {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config>;
}
impl ConfigWriter for KeyValueConfigService {
    fn write(&self, config: Config, mut to: &mut impl Write) -> std::io::Result<()> {
        for v in config.values {
            writeln!(&mut to, "{0}={1}", v.0, v.1).unwrap_err();
        }
        Ok(())
    }
}
impl ConfigReader for KeyValueConfigService {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config> {
        let mut buffer = String::new();
        from.read_to_string(&mut buffer)?;

        // chain iterators together and collect the results
        let values:HashMap<String,String> = buffer
            .split_terminator("\n") // split
            .map(|line| line.trim()) // remove whitespace
            .filter(|line| { // filter invalid lines
                let pos = line.find("=")
                    .unwrap_or(0);
                pos > 0 && pos < line.len() - 1
            })
            .map(|line| { // create a tuple from a line
                let parts = line.split("=")
                    .collect::<Vec<&str>>();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect(); // transform it into a vector
        Ok(Config::new(values))
    }
}

impl ValueGetter for Config {
    fn get(&self, s: &str) -> Option<String> {
        self.values.iter()
            .find_map(|tuple| if &tuple.0 == s {
                Some(tuple.1.clone())
            } else {
                None
            })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        let mut values = HashMap::new();
        values.insert("first".to_owned(), "second".to_owned());
        values.insert("first".to_owned(), "third".to_owned());
        let config = Config::new(values);
        let (_, value) = config.values.first().unwrap();
        assert_eq!(value, "third");
    }
}
