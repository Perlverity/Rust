rpn_calc_perl
=============

## RPN library for Perl

### Example
```Rust
let src = String::from("4 5 * 2 *");
let value = rpn_calc::eval(src).unwrap();
println!("value: {}", value);
```