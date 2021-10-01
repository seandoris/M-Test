use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Library{
    pub books: Vec<Book>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Book{
    pub title:String,
    pub author:String,
    pub pages:u32,
    pub status:String,
}

/*Function for turning the read contents of the json file into the 
structure Library */
fn tobook(json_in:&str) -> Library{
    let deserialized: Library = serde_json::from_str(json_in).unwrap();
    deserialized
}

/*Function for reading the contents of the library.json file */
pub fn read_library()->Vec<Book>{
    let mut file = File::open("library.json").expect("Unable to open library.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let library= tobook(contents.as_str());
    library.books
}