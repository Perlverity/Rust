fn main() {
    let s = "こんにちは";
    let ch = s.chars().nth(1).unwrap();
    println!("{}", ch);

    let pr = "猫に小判";
    for c in pr.bytes() {
        print!("{:2x} ", c);
    }
    println!("\nバイト数={}B", pr.len());

    let pr2 = "窮鼠猫を嚙む";

    for c in pr2.chars() {
        print!("[{}]", c);
    }
    println!("\n文字数={}文字", pr2.chars().count());

    let pr_chars: Vec<char> = pr2.chars().collect();

    for c in pr_chars.iter() {
        print!("({})", c);
    }
    println!("\n文字数={}文字", pr_chars.len());


    echo("愚かな人でも黙っていると");
    echo("賢い人に見えることがある");

    // let s = String::from("ECHO");
    // echo(&s);
}

fn echo(s: &'static str) {
    println!("{}", s);
}
