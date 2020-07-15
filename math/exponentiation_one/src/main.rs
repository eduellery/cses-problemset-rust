use std::io;

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
	.expect("Failed to read size line");
    let n: u64 = n.trim().parse().expect("Not a number");
    for _ in 0..n {
        let mut numbers = String::new();
        io::stdin()
            .read_line(&mut numbers)
	    .expect("Failed to read size line");
        let mut iter = numbers.split_whitespace();
        let a: u64 = iter.next().unwrap().parse().unwrap(); 
        let b: u64 = iter.next().unwrap().parse().unwrap();
	println!("{}", pow(a, b));
    }
}

fn mult(x: u64, y: u64) -> u64 {
    return (x * y) % MOD;
}

fn pow(b: u64, e: u64) -> u64 {
    // Exponentiation by squaring
    if e == 0 {
        return 1;
    } else if e % 2 == 0 {
        // Instead of b^e, (b^2)^(e/2)
	return pow(mult(b, b), e/2);
    } else {
        return mult(b, pow(b, e-1));
    }
}
