fn main() {
    let g1 = String::from("Hello");
    show_message(&g1);
    println!("{}", g1);

    let m = gen_message();
    println!("{}", m);

    let mut msg = String::from("Hello");
    println!("{}", msg);

    add_quote(&mut msg);
    println!("{}", msg);

    let mut v = 16;
    x2(&mut v);
    println!("{}", v);
}

fn show_message(message: &String) {
    println!("{}", message);
}

fn gen_message() -> String {
    let msg = String::from("Hello");
    return msg;
}

fn add_quote(msg: &mut String) {
    msg.insert(0, '[');
    msg.push(']');
}

fn x2(arg: &mut i32) {
    *arg = *arg * 2;
}