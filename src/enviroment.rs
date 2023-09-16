use std::collections::HashMap;

pub struct Env {
    stack: HashMap<String,i64>
}

impl Env {
    pub fn new() -> Self {
        Self { stack:HashMap::new() }
    }
    pub fn set_value(&mut self, key: String, value: i64) {
        self.stack.insert(key, value);
    }
    pub fn get_value(&self, key: String) -> Option<&i64> {
        self.stack.get(&key)
    }
}