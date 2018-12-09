use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn scan(s: &str) -> (i32, i32) {
    let mut map = HashMap::new();
    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    let mut twos = 0;
    let mut threes = 0;
    for (_, v) in &map {
        if *v == 2 {
            twos += 1;
        } else if *v == 3 {
            threes += 1;
        }
    }
    (twos, threes)
}

fn checksum(list: Vec<String>) -> i32 {
    let mut twos = 0;
    let mut threes = 0;
    for s in &list {
        let (i, j) = scan(s);
        if i > 0 {
            twos += 1;
        }
        if j > 0 {
            threes += 1;
        }
    }
    twos * threes
}

fn parse(filename: &str) -> Result<Vec<String>, io::Error> {
    let f = File::open(filename).expect("unable to openfile");
    let buffered = BufReader::new(f);

    let mut strs = Vec::new();
    for line in buffered.lines() {
        strs.push(line.expect("unable to read line"));
    }
    Ok(strs)
}

fn dist(a: &str, b: &str) -> i32 {
    let mut i = 0;
    let mut diffs = 0;
    while i < a.len() {
        if a.as_bytes()[i] != b.as_bytes()[i] {
            diffs += 1;
        }
        i += 1;
    }
    diffs
}

fn common(a: String, b: String) -> String {
    let mut i = 0;
    let mut buffer = Vec::new();
    while i < a.len() {
        let tmp = a.as_bytes()[i];
        if tmp == b.as_bytes()[i] {
            buffer.push(tmp);
        }
        i += 1;
    }
    String::from_utf8(buffer).unwrap()
}

fn traverse(list: Vec<String>) -> (String, String, i32) {
    let (head, body) = list.split_first().unwrap();

    let mut current_dist = 16;
    let mut most_similar = String::from("");
    for item in body.to_vec() {
        let d = dist(&head, &item);
        if d < current_dist {
            current_dist = d;
            most_similar = item;
        }
    }

    if body.len() > 1 {
        let (a, b, d) = traverse(body.to_vec());
        if d < current_dist {
            return (a, b, d);
        }
    }
    (head.to_string(), most_similar, current_dist)
}

fn main() {
    let strs = parse("./src/data").expect("unable to read strings");
    let (a, b, d) = traverse(strs.clone());

    println!("checksum {}", checksum(strs));
    println!("{} and {} are quite similar: {}", a, b, d);
    println!("common: {}", common(a, b));
}
