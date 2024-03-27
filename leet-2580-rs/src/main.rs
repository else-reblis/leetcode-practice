struct Solution;

pub const MOD: i32 = 1000000007;

impl Solution {
    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        let ranges = ranges.into_iter().map(|v| (v[0], v[1])).collect();
        let n_group = count_groups(ranges);

        pow_mod(2, n_group, MOD)
    }
}

fn count_groups(mut ranges: Vec<(i32, i32)>) -> u32 {
    let mut count = 1;

    ranges.sort();
    ranges.iter().fold(ranges[0].1, |max, this| {
        if max < this.0 {
            count += 1;
        }
        max.max(this.1)
    });

    count
}

fn pow_mod(mut a: i32, mut b: u32, m: i32) -> i32 {
    let mut res: i64 = 1;

    while b > 0 {
        if b % 2 == 1 {
            res = (res * a as i64) % m as i64;
        }

        a = ((a as i64).pow(2) % m as i64) as i32;
        b /= 2;
    }

    res as i32
}

fn main() {
    let res = Solution::count_ways(vec![
        vec![34, 56],
        vec![28, 29],
        vec![12, 16],
        vec![11, 48],
        vec![28, 54],
        vec![22, 55],
        vec![28, 41],
        vec![41, 44],
    ]);
    println!("res = {res}");
}
