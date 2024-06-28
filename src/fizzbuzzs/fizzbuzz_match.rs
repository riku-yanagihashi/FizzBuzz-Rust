pub fn fizzbuzz_match() {
    for num in 1..51 {
        match(num % 3, num % 5) {
            (0,0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", num)
        }
    }
}

