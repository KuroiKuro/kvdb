use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::{DataMap, DataValue};

    #[test]
    fn test_data_value_instantiation_str() {
        let d = DataValue::from("hello".to_string());
        let correct = DataValue::Str(Box::new("hello".to_string()));
        assert_eq!(d, correct);
    }

    #[test]
    fn test_data_value_instantiation_int() {
        let d = DataValue::from(20);
        let correct = DataValue::Int(Box::new(20));
        assert_eq!(d, correct);
    }

    #[test]
    fn test_data_value_instantiation_bigint() {
        let d = DataValue::from(11291_i64);
        let correct = DataValue::BigInt(Box::new(11291));
        assert_eq!(d, correct);
    }

    #[test]
    fn test_data_value_instantiation_bool() {
        let d = DataValue::from(true);
        let correct = DataValue::Bool(Box::new(true));
        assert_eq!(d, correct);
    }

    #[test]
    fn test_data_value_instantiation_float() {
        let d = DataValue::from(2.96_f32);
        let correct = DataValue::Float(Box::new(2.96));
        assert_eq!(d, correct);
    }

    #[test]
    fn test_data_value_instantiation_bigfloat() {
        let d = DataValue::from(41.66132);
        let correct = DataValue::BigFloat(Box::new(41.66132));
        assert_eq!(d, correct);
    }

    #[test]
    fn test_datamap() {
        let mut map = DataMap::new();
        let value1 = ("value1", DataValue::from(32_i32));
        let value2 = ("value2", DataValue::from(false));
        let value3 = ("value3", DataValue::from("My value".to_string()));

        map.insert(value1.0, value1.1.clone());
        map.insert(value2.0, value2.1.clone());
        map.insert(value3.0, value3.1.clone());

        let value1_val = map.get(value1.0).unwrap();
        assert_eq!(*value1_val, value1.1);
        let value2_val = map.get(value2.0).unwrap();
        assert_eq!(*value2_val, value2.1);
        let value3_val = map.get(value3.0).unwrap();
        assert_eq!(*value3_val, value3.1);
    }
}
