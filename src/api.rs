extern crate serde_json;

use self::serde_json::{Value, Error};
use self::serde_json::value::Index;
use self::serde_json::Number;

pub struct API {
    pub key: String,
    pub data: Value
}

impl API{
    pub fn new(key: String, data: Value) -> Self {
        API {
            key: key,
            data: data
        }
    }

    pub fn get<T: Index + Sized>(&self, el : T) -> Option<Self> {
        match self.data.get(el) {
            Some(data) => Some(API{
                key: self.key.clone(),
                data: data.clone()
            }),
            None => None
        }
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_api_creation(){
        let json_string = "[1, 2, 3, 4]".to_string();
        let g_son : Value = serde_json::from_str(&json_string[..]).unwrap();
        let api : API = API::new("id".to_string(), g_son);
        assert_eq!(api.key, "id".to_string());
    }

    #[test]
    fn test_api_get_on_array(){

    }
}

