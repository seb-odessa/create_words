use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn load(filename: &String) -> HashMap<String, String> {
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
            if !key.is_empty() {
                map.insert(key.clone().to_lowercase(), val.clone());
            }
            key.clear();
            val.clear();
        }
    }
    map
}

fn check_word(mut word: HashMap<char, i32>, maybe: &String) -> bool {
    for ch in maybe.chars() {
        let entry = word.entry(ch).or_insert(0);
        if *entry == 0 {
            return false;
        } else {
            *entry -= 1;
        }
    }
    return true;
}

fn check(word : &String, map: &HashMap<String, String>) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut chars: HashMap<char, i32> = HashMap::new();
    for ch in word.chars() {
        let entry = chars.entry(ch).or_insert(0);
        *entry += 1;
    }

    for (key, _) in map {
        if check_word(chars.clone(), &key) {
            words.push(key.clone());
        }
    }
    return words;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage {} WORD DICTIONARY", args[0]);
    }
    let word: String = args[1].clone().to_lowercase();
    println!("{}", &word);
    let map = load(&args[2]);
    for found in &check(&word, &map) {
        let word: String = found.clone();
        println!("{} {}", word, map[&word]);
    }
}