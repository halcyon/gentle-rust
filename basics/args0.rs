// args0.rs
fn main() {
    for arg in std::env::args() {
        println!("'{}'", arg);
    }

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len () > 0 {
        println!("{:?}", args)
    }
}
