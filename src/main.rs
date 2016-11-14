use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage {} WORD DICTIONARY", args[0]);
    }
    let word: String = args[1].clone().to_uppercase();
    println!("{}", &word);
    let dict = load(&args[2]);
    for word in &check_dict(&word, &dict) {
        let word: String = word.clone();
        println!("{} {}", word, dict[&word]);
    }
}

fn load(filename: &String) -> HashMap<String, String> {
    let mut dict: HashMap<String, String> = HashMap::new();
    if let Some(file) = File::open(&filename).ok() {
        let mut key = String::with_capacity(32);
        let mut val = String::with_capacity(256);
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let mut first = true;
            for ch in line.unwrap().chars() {
                if first {
                    if ch.is_alphabetic() {
                        key.push(ch);
                    } else {
                        first = false;
                    }
                } else {
                    val.push(ch);
                }
            }
            if !key.is_empty() {
                dict.insert(key.clone().to_uppercase(), val.clone());
            }
            key.clear();
            val.clear();
        }
    }
    return dict;
}

fn check_word(mut chars: HashMap<char, i32>, word: &String) -> bool {
    let mut word: Vec<char> = word.chars().collect();
    word.sort();
    for ch in word {
        let entry = chars.entry(ch).or_insert(0);
        if *entry == 0 {
            return false;
        } else {
            *entry -= 1;
        }
    }
    return true;
}

fn check_dict(word : &String, dict: &HashMap<String, String>) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut chars: HashMap<char, i32> = HashMap::new();
    for ch in word.chars() {
        let entry = chars.entry(ch).or_insert(0);
        *entry += 1;
    }

    for (word, _) in dict {
        if check_word(chars.clone(), &word) {
            words.push(word.clone());
        }
    }
    return words;
}

