use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

static CONF_DIR: &'static str = "./conf";

#[derive(Debug)]
pub struct Configuration {
    name: String,
    // filename: String,
    properties: HashMap<String,String>,
}

impl Configuration {

    pub fn new(name: String) -> Configuration {
        let mut conf = Configuration {
                name: name,
                // filename: String::from(""),
                properties: HashMap::new(),
            };
        conf.properties = conf.load_from_file();
        conf
    }

    pub fn filename(&self) -> String {
        format!("{}/{}.conf", CONF_DIR, self.name)
    }

    pub fn load_from_file(&self) -> HashMap<String,String> {
        let f = match File::open(self.filename()) {
            Ok(file) => file,
            Err(error) => {
                panic!("Failed to open file '{}' Error: {:?}.", self.filename(), error)
            },
        };
        let file = BufReader::new(&f);
        let mut conf_map: HashMap<String,String> = HashMap::new();
        for (num, line) in file.lines().enumerate() {
            let l = line.unwrap();
            println!("Line {}: {}", num, l);
            let opt: Vec<&str> = l.split("=").collect();
            println!("Key: {} Val: {}", opt[0], opt[1]);
            conf_map.insert(String::from(opt[0]), String::from(opt[1]));
        }
        conf_map
    }

    pub fn count(&self) -> usize {
        self.properties.len()
    }

    pub fn get_conf(&self, name: &String) -> &String {
        self.properties.get(name).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conf_test() {
        let test_conf = Configuration::new(String::from("test"));
        assert_eq!(test_conf.count(), 1);

        assert_eq!(test_conf.get_conf(&String::from("test.key")), "test.value");
    }
}
