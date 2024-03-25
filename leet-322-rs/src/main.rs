struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        coin_change(coins, amount)
    }
}

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut res = vec![None; (amount + 1) as usize];
    res[0] = Some(0);

    for i in 1..=amount {
        res[i as usize] = coins
            .iter()
            .filter(|coin| **coin <= i)
            .filter_map(|coin| {
                let j = (i - *coin) as usize;
                if let Some(res) = res[j] {
                    Some(res + 1)
                } else {
                    None
                }
            })
            .min();
    }

    res[amount as usize].unwrap_or(-1)
}

fn main() {
    let res = Solution::coin_change(vec![1, 2, 5], 11);
    println!("res = {res}");
}
