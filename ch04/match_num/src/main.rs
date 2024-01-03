fn main() {
    let i = 6u8;
    match i {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"),
    }

    let age = 24;
    let age_str = match age {
        0 => "乳児",
        1..=5 => "幼児",
        6..=12 => "小学生",
        _ => "大人",
    };
    println!("{}歳は{}料金", age, age_str);
}
