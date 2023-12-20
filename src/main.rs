#[macro_use]
extern crate rocket;

use once_cell::sync::Lazy;
use std::fs::File;
use std::io::BufRead; // Import the BufRead trait
use std::io::{BufReader, Result};
use std::{collections::HashMap, sync::Mutex};
use rocket::http::Status;


static mut _HASHMAP: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn check_lines(_map: &mut HashMap<String, String>, text: &str) -> Result<bool> {
    if _map.is_empty() {
        let mut lines = Vec::new();
        let file = File::open("src/list.txt")?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            // Call the lines method on the BufReader object
            let line = line?;
            lines.push(line.clone());
            _map.insert(line.clone(), line.clone());
        }
        println!("{}", "Map is empty")
    }
    if _map.contains_key(text) {
        println!("{} is already in the list", text);
        return Ok(true);
    } else {
        println!("{} is not in the list", text);
        return Ok(false);
    }
}

#[get("/api/<url>")]
fn index(url:&str) -> &'static str {
    let a = check_lines(unsafe { &mut *_HASHMAP.lock().unwrap() }, url);
    match a {
        Ok(true) => return "{is_safe : false}",
        Ok(false) => return "{is_safe : true}",
        Err(_) => return "Error"
    }

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}