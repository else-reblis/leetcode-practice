struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut count_1 = 0;
        let n = s.len();

        s.chars().into_iter().for_each(|c| if c == '1' {count_1 += 1});

        if count_1 == 0 {
            "0".repeat(n)   // not useful anyway
        } else {
            "1".repeat(count_1 - 1) + &"0".repeat(n - count_1) + "1"
        }
    }
}

fn main() {
    let res = Solution::maximum_odd_binary_number("001".to_string());
    println!("{}", res);
}
