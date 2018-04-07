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

    fn valid(&self) -> bool {
        //look up each item to tests resolving references, want any panics to boil up on init
        for (prop, _val) in self.properties.iter() {
            self.get_conf(prop);
        }
        true
    }

    pub fn new(name: String) -> Configuration {
        let mut conf = Configuration {
                name: name,
                // filename: String::from(""),
                properties: HashMap::new(),
            };
        conf.properties = conf.load_from_file();
        conf.valid();
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
        //resolves references recursively, else should panic.
        let temp_value = match self.properties.get(name) {
            Some(value) => value,
            None => panic!("Unable to resolve reference ${{{}}}, in conf file {}", name, self.filename())
        };
        println!("TempValue: {}", temp_value);
        let char_vec: Vec<char> = temp_value.chars().collect();
        if char_vec[0] == '$' {
            if char_vec[1] == '{' {
                let len = char_vec.len();
                let new_key = &temp_value[2..len-1];
                println!("New key: {}", new_key);
                return self.get_conf(&String::from(new_key));
            }
        }
        temp_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conf_test() {
        let test_conf = Configuration::new(String::from("test"));
        assert!(test_conf.count() > 0);

        assert_eq!(test_conf.get_conf(&String::from("test.key")), "test.value");
        assert_eq!(test_conf.get_conf(&String::from("test.key2")), "test.value");
    }

    #[test]
    #[should_panic]
    fn bad_conf_test() {
        Configuration::new(String::from("test-bad"));
    }
}
