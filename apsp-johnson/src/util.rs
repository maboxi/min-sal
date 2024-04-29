use std::str::FromStr;
use std::io::{stdin, stdout, Write};
use core::fmt::Debug;

pub fn get<T: FromStr>(preamble: String, default: T) -> T where <T as FromStr>::Err: Debug {
    print!("{}", preamble);
    stdout().flush().expect("Error flushing stdout!");
    let mut userinput = String::new();
    stdin().read_line(&mut userinput).expect("Error reading user input!");
    userinput.trim().parse::<T>().unwrap_or(default)
}