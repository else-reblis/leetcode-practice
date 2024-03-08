use std::ops::RangeInclusive;

pub const M: i64 = 1000000007;

struct Solution;

impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        if n == 1 {
            1
        } else {
            minimum_possible_sum(n as i64, target as i64) as i32
        }
    }
}

pub fn minimum_possible_sum(n: i64, target: i64) -> i64 {
    let t = target;
    let m = target / 2;

    if n <= m {
        sum_range(1..=n)
    } else {
        let d = n - m;
        let s0 = sum_range(1..=m);
        let s1 = sum_range(t..=t+d-1);
        dbg!(d, s0, t, t + d - 1);
        (s0 + s1) % M
    }
}

pub fn sum_range(range: RangeInclusive<i64>) -> i64 {
    let m = *range.start();
    let n = *range.end();
    let d = n - m + 1;
    ((m + n) * d / 2) % M
}

fn main() {
    let res = Solution::minimum_possible_sum(16, 6);
    println!("res = {res}");
}
