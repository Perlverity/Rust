fn main() {
    let stc = "2 3 *".to_string();
    let ans = rpn_calc_perl::eval(stc).unwrap();

    println!("{}", ans);
}
