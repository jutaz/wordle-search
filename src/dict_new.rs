use fst::{IntoStreamer, Set};
use regex_automata::dense;
use regex::Regex;

use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn search (search: Vec<String>, must_have: Vec<char>) -> Result<usize, Box<dyn std::error::Error>> {

    let reader = BufReader::new(File::open("words.txt").expect("Cannot open file.txt"));
    let mut items: Vec<String> = Vec::new();

    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            items.push(word.to_string())
        }
    }

    items.sort();

    let set = Set::from_iter(items)?;

    let dense_dfa = dense::Builder::new()
        .anchored(true)
        .build(&search.join(""))?;

    let dfa = match dense_dfa {
        dense::DenseDFA::PremultipliedByteClass(dfa) => dfa,
        _ => unreachable!(),
    };

    // Apply our fuzzy query to the set we built.
    let stream = set.search(&dfa).into_stream();

    let strs = stream.into_strs()?;

    let keys: Vec<&String> = strs
        .iter()
        .filter(|word| must_have.iter().all(|c| word.to_owned().contains(c.to_owned())))
        .collect::<Vec<&String>>();
    let length = keys.len();

    for word in keys.into_iter() {
        println!("{}", word);
    }

    Ok(length)
}
