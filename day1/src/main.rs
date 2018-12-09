use std::error;
use std::fmt;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone)]
struct UnknownSignError;
impl fmt::Display for UnknownSignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown sign")
    }
}
impl error::Error for UnknownSignError {
    fn description(&self) -> &str {
        "unknown sign"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn parse(filename: &str) -> Result<Vec<i32>, io::Error> {
    let f = File::open(filename)
        .expect("unable to open file");
    let buffered = BufReader::new(f);

    let mut changes: Vec<i32> = Vec::new();
    for line in buffered.lines() {
        let s= line.expect("unable to read line");
        let sign: i32 = match &s[0..1] {
            "+" => Ok(1),
            "-" => Ok(-1),
            _ => Err(UnknownSignError) ,
        }.unwrap();
        let number:i32 = s[1..].parse().unwrap();

        changes.push(number * sign);
    }

    Ok(changes)
}

fn already_seen(frequency: i32, frequencies: &Vec<i32>) -> bool {
    for i in frequencies {
        if *i == frequency {
            return true;
        }
    }
    false
}

fn main() {
    let filename = "./src/data";
    let changes = parse(filename).expect("unable to parse");
    //let changes : Vec<i32> = [1,-2,3,1,1,-2].to_vec();
    //let changes : Vec<i32> = [1,-1].to_vec();
    //let changes : Vec<i32> = [3,3,4,-2,-4].to_vec();
    //let changes : Vec<i32> = [-6,3,8,5,-6].to_vec();
    //let changes : Vec<i32> = [7,7,-2,-7,-4].to_vec();

    let mut frequency = 0;
    let mut frequencies = Vec::new();
    frequencies.push(0);

    let mut found = false;
    while !found {
        for change in &changes {
            frequency = frequency + change;
            if already_seen(frequency, &frequencies) {
                found = true;
                break
            }
            frequencies.push(frequency)
        }
    }
    println!("{}", frequency)
}
