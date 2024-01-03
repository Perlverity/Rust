peg::parser! ( grammar calc() for str {
    pub rule eval() -> i64
    = expr()

    rule expr() -> i64
    = l:term() "+" r:expr() { l + r }
    / l:term() "-" r:expr() { l - r }
    / term()

    rule term() -> i64
    = l:value() "*" r:term() { l * r }
    / l:value() "/" r:term() { l / r }
    / value()

    rule value() -> i64
    = number()
    / "(" e:expr() ")" { e }

    rule number() -> i64
    = value:$(['0'..='9']+) { value.parse().unwrap() }
});

fn main() {
    println!("{}", calc::eval("2+5*3").unwrap());
    println!("{}", calc::eval("(8+2)*3").unwrap());
    println!("{}", calc::eval("200/2+50").unwrap());
}
