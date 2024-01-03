fn main() {
    hex_dump("自分の口をふさぐのは、自分の手だ。");

    let pr = "知恵は武器よりも価値がある。";
    println!("先頭2文字: {}", &pr[0..6]);
    println!("4-5文字目: {}", &pr[9..15]);

    let mut sub1 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i < 2 { sub1.push(c); continue; }
        break;
    }
    println!("先頭2文字: {}", sub1);

    let mut sub2 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if 3 <= i && i <= 4 { sub2.push(c); }
    }
    println!("4-5文字目: {}", sub2);

    let sub3: String = pr.chars().take(2).collect();
    println!("先頭2文字: {}", sub3);

    let pr_chars: Vec<char> = pr.chars().collect();
    let sub_chars = &pr_chars[3..=4];
    let sub4: String = sub_chars.into_iter().collect();
    println!("4-5文字目: {}", sub4);


    let s = "隣の客はよく柿食う客だ";

    match s.find('柿') {
        Some(i) => println!("柿={}B", i),
        None => println!("柿はない"),
    }

    match s.find("バナナ") {
        Some(i) => println!("バナナ={}B", i),
        None => println!("バナナはない"),
    }


    let s1 = format! ("{}{}", "There is more happiness in giving ", "than there is in receiving.");
    let res = s1.find(|c:char| c.to_ascii_uppercase() == 'S');
    match res {
        Some(i) => println!("{}B", i),
        None => println!("None"),
    }


    let s2 = "苦しむ人にはどの日も悪い日である。";
    let s2 = s2.replace("苦しむ", "楽しむ");
    let s2 = s2.replace("悪い", "良い");
    println!("{}", s2);


    let zipcode = "105-0011";

    println!("-- スライス --");
    println!("前半: {}", &zipcode[..3]);
    println!("後半: {}", &zipcode[4..]);

    println!("-- split_at --");
    let (zip1, zip2) = zipcode.split_at(3);
    let (zip2, zip3) = zip2.split_at(1);
    println!("前半: {}", zip1);
    println!("記号: {}", zip2);
    println!("後半: {}", zip3);

    println!("-- split_off --");
    let mut zip1 = String::from(zipcode);
    let mut zip2 = zip1.split_off(3);
    let zip3 = zip2.split_off(1);
    println!("前半: {}", zip1);
    println!("記号: {}", zip2);
    println!("後半: {}", zip3);

    println!("-- split --");
    let zip_a: Vec<&str> = zipcode.split('-').collect();
    println!("前半: {}", zip_a[0]);
    println!("後半: {}", zip_a[1]);

}

fn hex_dump(s: &str) {
    for (i, c) in s.bytes().enumerate() {
        if i % 16 == 0 {
            print!("{:08x}|", i);
        }
        if i % 4 == 3 {
            print!("{:02x}| ", c);
        } else {
            print!("{:02x} ", c);
        }
        if i % 16 == 15 {
            println!("");
        }
    }
    println!("");
}
