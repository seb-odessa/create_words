use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn load(filename: &String) ->Option<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();
    if let Some(file) = File::open(&filename).ok() {
        let mut key = String::with_capacity(32);
        let mut val = String::with_capacity(256);
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let mut word = true;
            for ch in line.unwrap().chars() {
                if word {
                    if ch.is_alphabetic() {
                        key.push(ch);
                    } else {
                        word = false;
                    }
                } else {
                    val.push(ch);
                }
            }
            if !val.is_empty() && !val.is_empty() {
                map.insert(key.clone(), val.clone());
            }
            key.clear();
            val.clear();
        }
    }
    Some(map)
}


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage {} WORD DICTIONARY", args[0]);
    }

    if let Some(map) = load(&args[2]) {
        for (key, val) in &map {
                println!("{}", key);
            }
    }
    println!("{} => :", &args[1]);
}