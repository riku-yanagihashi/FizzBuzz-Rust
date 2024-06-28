// これ勉強する必要ないです。死んでください
pub fn fizzbuzz_closure() {
    let fizzbuzz = |n: i32| -> String {
            let mut result = String::new();
            if n % 3 == 0 {
                result.push_str("Fizz");
            }
            if n % 5 == 0 {
                result.push_str("Buzz");
            }
            if result.is_empty() {
                result = n.to_string();
            }
            result
        };

        for i in 1..=100 {
            println!("{}", fizzbuzz(i));
        }
}
