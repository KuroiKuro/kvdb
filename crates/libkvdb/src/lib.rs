use std::collections::HashMap;

pub enum DataValue {
    Str(Box<String>),
    Int(Box<i32>),
    BigInt(Box<i64>),
    Bool(Box<bool>),
    Float(Box<f32>),
}

pub struct DataMap {
    map: HashMap<String, DataValue>
}

impl DataMap {
    pub fn new() -> Self {
        let map = HashMap::new();
        Self { map }
    }
}

impl Default for DataMap {
    fn default() -> Self {
        Self::new()
    }
}


