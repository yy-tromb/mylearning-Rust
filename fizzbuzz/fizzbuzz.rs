fn main() {
    for i in 1..100 {
        println!(
            "{}",
            match (i % 3, i % 5) {
                (0, 0) => "FizzBuzz".into(),
                (0, _) => "Fizz".into(),
                (_, 0) => "Buzz".into(),
                _ => i.to_string(),
            }
        )
    }
}
