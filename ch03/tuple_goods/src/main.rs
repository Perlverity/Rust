struct Item(String, i64);

fn main() {
    let banana = Item("banana".to_string(), 300);
    let apple = Item("apple".to_string(), 200);
    let mango = Item("mango".to_string(), 400);

    let items = vec![banana, apple, mango];

    let total = print_and_sum_items(&items);
    println!("Total: {}", total);

    let points = [80, 90, 100, 70, 60];
    println!("{:?}", points);
    println!("len={}", points.len());

    let s = String::from("beep");
    let ss = &s[0..3];
    println!("{}", ss);

    let a = [0, 1, 2, 3, 4, 5];
    let a_slice = &a[0..3];
    println!("{:?}", a_slice);
    println!("{:?}", &a[3..5]);
    println!("{:?}", &a[4..6]);

    let b = [1,2,3,4,5,6,7,8,9,10];
    println!("a={}", sum_slice(&b[..]));

    let c = vec![1,2,3,4,5,6,7,8,9,10];
    println!("b={}", sum_slice(&c[..]));
}

fn print_tuple(item: &Item) {
    println!("{}を{}円で購入", item.0, item.1);
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for item in items {
        print_tuple(&item);
        total += item.1;
    }
    total
}

fn sum_slice(items: &[i64]) -> i64 {
    let mut total = 0;
    for item in items {
        total += item;
    }
    total
}