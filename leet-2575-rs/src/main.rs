struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        divisibility_array_fp_style(word, m as u64)
    }
}

pub fn divisibility_array_fp_style(word: String, m: u64) -> Vec<i32> {
    let mut rem: u64 = 0;

    word.chars().map(|c| {
        let d = c.to_digit(10).unwrap() as u64;
        rem = (rem * 10 + d) % m;

        if rem % m == 0 {
            1
        } else {
            0
        }
    }).collect()
}

fn main() {
    let res = Solution::divisibility_array("998244353".to_string(), 3);
    println!("{:?}", res);
}
