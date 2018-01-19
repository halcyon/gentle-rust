#![allow(unused_variables)]
fn main() {
    let s = "¡";
    println!("{}", &s[0..1]); // bad, first byte of a multibyte character
    let q = [1, 2, 3, 4];
    println!("{:?}", &q[0..4]);
}
