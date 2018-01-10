// fun1.rs

fn sqr(x: f64) -> f64 {
    x * x
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let res = sqr(2.0);
    println!("square is {}", res);
    println!("factorial is {}", factorial(3));

    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2);
    let mut res3 = 0.0;
    modifies(&mut res3);
    println!("res is {}", res3)
}
