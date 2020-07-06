use std::io;
use std::collections::HashSet;

fn main() {
    let mut n = String::new();
    let mut v = String::new();

    io::stdin()
        .read_line(&mut n)
	.expect("Failed to read size line");

    io::stdin()
        .read_line(&mut v)
	.expect("Failed to read values line");

    let s = v.trim().split(' ').flat_map(str::parse::<i32>).collect::<HashSet<_>>();

    println!("{}", s.len());
}
