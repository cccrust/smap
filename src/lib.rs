use std::collections::HashMap;

pub struct Dict {
    map: HashMap<String, String>,
}

impl Dict {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn add(&mut self, k:&str, v:&str) {
        self.map.insert(k.to_string(), v.to_string());
    }

    pub fn get(&mut self, k:&str)->Option<&String> {
        return self.map.get(k);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    */
    #[test]
    fn dict_test() {
        let mut e2c = Dict::new();
        e2c.add("a", "一隻");
        e2c.add("dog", "狗");
        e2c.add("cat", "貓");
        e2c.add("chase", "追");
        e2c.add("bite", "咬");
        assert!(e2c.get("cat") != None);
        assert!(e2c.get("xxx") == None);
    }
}
