use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub fn input<T>(path: &str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(path).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().enumerate().map(|(index, l)| {
        l.expect("Error reading line")
            .parse::<T>()
            .expect(&format!("Error while parsing line {}", index+1))
    })
}
