use std::collections::HashMap;

pub enum DataValue {
    Str(Box<String>),
    Int(Box<i32>),
    BigInt(Box<i64>),
    Bool(Box<bool>),
    Float(Box<f32>),
    BigFloat(Box<f64>),
}

impl From<String> for DataValue {
    fn from(value: String) -> Self {
        Self::Str(Box::new(value))
    }
}

impl From<i32> for DataValue {
    fn from(value: i32) -> Self {
        Self::Int(Box::new(value))
    }
}

impl From<i64> for DataValue {
    fn from(value: i64) -> Self {
        Self::BigInt(Box::new(value))
    }
}

impl From<bool> for DataValue {
    fn from(value: bool) -> Self {
        Self::Bool(Box::new(value))
    }
}

impl From<f32> for DataValue {
    fn from(value: f32) -> Self {
        Self::Float(Box::new(value))
    }
}

impl From<f64> for DataValue {
    fn from(value: f64) -> Self {
        Self::BigFloat(Box::new(value))
    }
}

pub struct DataMap {
    map: HashMap<String, DataValue>
}

impl DataMap {
    pub fn new() -> Self {
        let map = HashMap::new();
        Self { map }
    }

    pub fn insert(&mut self, key: &str, value: DataValue) {
        self.map.insert(key.to_string(), value);
    }

    pub fn get(&mut self, key: &str) -> Option<&DataValue> {
        self.map.get(key)
    }
}

impl Default for DataMap {
    fn default() -> Self {
        Self::new()
    }
}
