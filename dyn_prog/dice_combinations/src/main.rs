use std::io;

const MOD: i32 = 1_000_000_007;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
	.expect("Failed to read size line");
    let size: i32 = n.trim().parse().expect("Not a number");

    let mut dp: Vec<i32> = vec![-1; (size+1) as usize];
    let ans = calc_dp(size, &mut dp);

    println!("{}", ans);
}

fn add(x: i32, y: i32) -> i32 {
    return (x + y) % MOD;
}

fn calc_dp(n: i32, dp: &mut Vec<i32>) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }
    if dp[n as usize] >= 0 {
        return dp[n as usize];
    }
    let mut ans: i32 = 0;
    for roll in 1..7 {
        ans = add(ans, calc_dp(n-roll, dp));
    }
    dp[n as usize] = ans;
    return ans;
}
