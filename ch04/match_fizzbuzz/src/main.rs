fn main() {
    for i in 1..=100 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i),
        }
    }

    for i in 1..=100 {
        let msg = match i {
            n if n % 15 == 0 => "FizzBuzz".to_string(),
            n if n % 3 == 0 => "Fizz".to_string(),
            n if n % 5 == 0 => "Buzz".to_string(),
            _ => format!("{}", i),
        };
        println!("{}", msg);
    }
}
