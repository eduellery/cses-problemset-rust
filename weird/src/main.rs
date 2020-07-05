use std::io;
 
fn main() {
    let mut n = String::new();
 
    io::stdin()
        .read_line(&mut n)
	.expect("Failed to read line");
 
    let mut n: u64 = n.trim().parse().expect("Not a number!");
 
    while n != 1 {
        print!("{} ", n);
        if n % 2 == 0 {
	    n /= 2;
	} else {
	    n = n * 3 + 1;
	}
    }
    println!("1");
}
