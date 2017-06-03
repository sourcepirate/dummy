extern crate serde_json;
extern crate hyper;

use self::serde_json::Value;
use self::serde_json::value::Index;

use self::hyper::uri::RequestUri;

fn safe_usize(val: &str) -> Option<usize>{
    match val.trim().parse::<usize>(){
        Ok(x) => Some(x),
        Err(_) => None
    }
}


#[derive(Debug, Clone, PartialEq)]
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
            Some(data) => Some(API {
                key: self.key.clone(),
                data: data.clone()
            }),
            None => None
        }
    }
    
    pub fn get_resource<T: Index + Sized>(&self, el: T) -> Option<Self>{
        match self.data {
           Value::Object(_) => self.get(el),
           _ => None
        }
    }


    pub fn get_by_path(&self, path: RequestUri) -> Result<String, &'static str>{
        if let RequestUri::AbsolutePath(loc) = path {
            let loc_v = loc.split("/").filter(|x| x.len() > 0).collect();
            let api_obj : API = self.navigate_out(loc_v).unwrap();
            Ok(api_obj.data.to_string())
        } else{
            Err("Only absolute path for now")
        }
    }

    pub fn get_by_index(&self, val: Option<usize>) -> Self {
        if !val.is_none(){
             match self.get(val.unwrap()) {
                 Some(x) =>  x,
                 None => API {
                     key: self.key.clone(),
                     data: Value::Array(vec![])
                 }
             }
        } else {
            API {
                key: self.key.clone(),
                data: Value::Array(vec![])
            }
        }
    }

    pub fn navigate_out(&self, routing: Vec<&str>) -> Option<Self> {
        let mut value = self.clone();
        for route in routing{
            value = match value.get_resource(route) {
                Some(x) => x,
                None => value.get_by_index(safe_usize(route))
            };
        }
        Some(value)
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
        let json_string = "[1, 2, 3, 4]".to_string();
        let g_son: Value = serde_json::from_str(&json_string[..]).unwrap();
        let api: API = API::new("id".to_string(), g_son);
        let child: API = api.get(0).unwrap();
        assert_eq!(child.key, api.key);
    }
    
    #[test]
    fn test_api_get_on_object(){
        let json_string = "{\"hello\": \"[1, 2, 3, 4]\"}".to_string();
        let g_son: Value = serde_json::from_str(&json_string[..]).unwrap();
        let api : API = API::new("id".to_string(), g_son);
        let child: API = api.get("hello").unwrap();
        assert_eq!(child.key, api.key);
    }
    
    #[test]
    fn test_api_get_resource_on_object(){
       let json_string = "{\"hello\": [1, 2, 3, 4]}".to_string();
       let comp_string = "[1, 2, 3, 4]".to_string(); 
       let g_son: Value = serde_json::from_str(&json_string[..]).unwrap();
       let l_son: Value = serde_json::from_str(&comp_string[..]).unwrap();
       let api : API = API::new("id".to_string(), g_son);
       let child: API = api.get_resource("hello").unwrap();
       assert_eq!(child.data, l_son);
    }
    
    #[test]
    fn test_api_get_resource_on_array(){
       let json_string = "[1, 2, 3, 4]".to_string();
       let g_son: Value = serde_json::from_str(&json_string[..]).unwrap();
       let api : API = API::new("id".to_string(), g_son);
       let child: Option<API> = api.get_resource(0);
       match child {
           Some(_) => assert!(false),
           None => assert!(true)
       }
    }
}