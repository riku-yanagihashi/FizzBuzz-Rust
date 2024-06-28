mod fizzbuzzs;
use fizzbuzzs::fizzbuzz_while;
use fizzbuzzs::fizzbuzz_match;
use proconio::input;
use fizzbuzzs::fizzbuzz_closure;

fn main() {
    input! {
        x: String,
    }

    input(x);

}

fn input(w: String) {
    if w.contains("while") {
        fizzbuzz_while::fizzbuzz1();
    } else if w.contains("match") {
        fizzbuzz_match::fizzbuzz_match();
    } else if w.contains("closure") {
        fizzbuzz_closure::fizzbuzz_closure();
    } else {
        println!("ざんねん死んでください")
    }
}


