mod fizzbuzzs;
use fizzbuzzs::fizzbuzz_while;
use fizzbuzzs::fizzbuzz_match;
use std::io;

fn main() {
    let mut w = String::new();
    io::stdin().read_line(&mut w).expect("Failed to read line.");

    print!("{}", w);
    
    if w.contains("while") {
        println!("あなたしにたい？" );
        fizzbuzz_while::fizzbuzz1();
    } else if w.contains("match") {
        fizzbuzz_match::fizzbuzz_match();
    } else {
            println!("ざんねん死んでください")
    }
}



