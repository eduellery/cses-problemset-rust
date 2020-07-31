use std::io;

const MOD: i64 = 1_000_000_007;
const UPPER_BOUND: usize = 1_000_000;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
	.expect("Failed to read size line");
    let n: i32 = n.trim().parse().expect("Not a number");

    let mut f: Vec<i64> = vec![0; UPPER_BOUND+1];
    f[0] = 1;
    for i in 1..UPPER_BOUND+1 {
        f[i] = mul(i as i64, f[i-1]);
    }

    let mut fi: Vec<i64> = vec![0; UPPER_BOUND+1];    
    fi[UPPER_BOUND] = inv(f[UPPER_BOUND]);
    for i in (0..UPPER_BOUND).rev() {
        fi[i] = mul(fi[i+1], (i+1) as i64);
    }

    for _ in 0..n {
        let mut numbers = String::new();
        io::stdin()
            .read_line(&mut numbers)
	    .expect("Failed to read size line");
        let mut iter = numbers.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let ans: i64 = mul(f[a], mul(fi[b], fi[a-b]));
        println!("{}", ans);
    }
}

#[allow(dead_code)]
fn add(x: i64, y: i64) -> i64 {
    return (x + y) % MOD;
}

fn mul(x: i64, y: i64) -> i64 {
    return (x * y) % MOD;
}

fn pow(b: i64, e: i64) -> i64 {
    if e == 0 { return 1; }
    if e%2 == 0 { return pow(mul(b,b), e/2); }
    return mul(b, pow(b, e-1));
}

fn inv(x: i64) -> i64 {
    return pow(x, MOD-2);
}

#[allow(dead_code)]
fn div(x: i64, y: i64) -> i64 {
    return mul(x, inv(y));
}
