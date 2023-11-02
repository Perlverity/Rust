fn kakezan(a: i64, b: i64) -> i64 {
    a * b
}

fn main() {
    let ex1 = kakezan(3, 5);
    println!("3*5={}", ex1);

    let x2 = |n| n*2;
    println!("{}", x2(2));
    println!("{}", x2(8));
}