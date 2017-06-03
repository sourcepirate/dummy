extern crate serde_json;

use std::fs::File;
use std::path::{PathBuf};
use std::env::{current_dir};
use std::io::BufReader;
use std::io::prelude::*;
use api::API;
use self::serde_json::{from_str, Value};


pub struct Reader{
    pub path: PathBuf,
    pub id: String
}

impl Reader{
    pub fn new(id: String, p: String) -> Self {
       let path_buf :PathBuf = PathBuf::from(&p[..]);
       let cwd :PathBuf = current_dir().unwrap();
       let current_path = path_buf.as_path();
       let new_path :PathBuf = cwd.join(current_path);
       Reader{
           id: id,
           path: new_path
       }
    }
    
    pub fn exists(&self) -> bool {
        self.path.exists() && self.path.is_file()
    }
    
    pub fn contents(&self) -> Result<API, &'static str> {
       if !self.exists(){
          Err("File does not exist")
       }
       else{
       let filepath = self.path.to_str().unwrap().to_string();
       let file_obj :File = File::open(&filepath[..]).unwrap();
       let mut buf = BufReader::new(file_obj);
       let mut str_buf = String::new();
       match buf.read_to_string(&mut str_buf){
           Ok(_) => {
               let value: Value = from_str(&str_buf[..]).unwrap();
               let api :API = API::new(self.id.clone(), value.clone());
               Ok(api.clone())
           },
           Err(_) => Err("Error Reading")
       }
      }
    } 
}