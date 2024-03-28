struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        solution(next_visit)
    }
}

fn solution(next_visit: Vec<i32>) -> i32 {
    let n = next_visit.len();
    let mut s: Vec<i64> = vec![1; n + 1];

    for i in 0..n {
        s[i + 1] = m(2 * s[i] - s[next_visit[i] as usize] + 2);
    }

    m(s[n - 1] - 1) as i32
}

#[inline]
fn m(n: i64) -> i64 {
    (n + MOD) % MOD
}

fn main() {
    let res = solution(vec![0, 0, 2]);
    println!("res = {:?}", res);

    let res = Solution::first_day_been_in_all_rooms(vec![
        0, 0, 1, 2, 4, 0, 1, 6, 0, 0, 2, 3, 4, 3, 4, 11, 6, 0, 16, 14, 20, 16, 9, 9, 1, 8, 8, 4,
        14, 13, 5, 12, 8, 18, 27, 34, 36, 13, 10, 35, 13, 31, 13, 29, 2, 45, 17, 30, 10, 18, 41,
        14, 41, 22, 2, 4, 1, 15, 27, 35, 12, 10, 46, 25, 61, 8, 65, 57, 48, 61, 8, 35, 2, 66, 47,
        5, 54, 76, 73, 51, 13, 64, 15, 2,
    ]);
    println!("res = {:?}", res);
}
