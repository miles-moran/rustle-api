use serde::{Serialize, Deserialize};
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Words {
    pub words: Vec<String>
}


pub fn get_words(path:&str) -> Vec<String>{
    let v: Words = serde_json::from_str(path).unwrap();
    return v.words;
}