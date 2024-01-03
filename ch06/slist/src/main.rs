mod slist;

fn main() {
    let mut list = slist::List::new();

    list.push(100);
    list.push(200);

    list.unshift(300);
    list.unshift(400);

    println!("{}", list.get(0).unwrap());
    println!("{}", list.get(1).unwrap());
    println!("{}", list.get(2).unwrap());
    println!("{}", list.get(3).unwrap());
}
