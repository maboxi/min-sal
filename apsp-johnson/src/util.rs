use std::str::FromStr;
use std::io::{stdin, stdout, Write};
use core::fmt::Debug;
use crate::distance::Distance;

pub fn get<T: FromStr>(preamble: String, default: T) -> T where <T as FromStr>::Err: Debug {
    print!("{}", preamble);
    stdout().flush().expect("Error flushing stdout!");
    let mut userinput = String::new();
    stdin().read_line(&mut userinput).expect("Error reading user input!");
    userinput.trim().parse::<T>().unwrap_or(default)
}


pub fn print_matrix<T: std::fmt::Display>(matrix: &Vec<Distance<T>>, n: usize) {
    print!("\n{: >5} |", "");
    (0..n).for_each(|i| print!(" {: >5}", i));
    println!("\n{:->1$}", "", 8 + 2 + 6*n);

    for u in 0..n {
        println!("{: >5} |", "");
        print!("{: >5} |", u);

        for v in 0..n {
            print!(" {: >5}", matrix[u*n + v].num_or_inf());
        }

        println!("");
        println!("{: >5} |", "");
    }

}