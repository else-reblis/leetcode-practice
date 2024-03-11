use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        k_sum(nums, k)
    }
}

pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
    let sum: i64 = nums.iter().map(|x| *x as i64).filter(|&x| x > 0).sum();
    let mut nums = nums
        .into_iter()
        .map(|i| i.abs() as i64)
        .collect::<Vec<i64>>();

    nums.sort();

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(nums[0] as i64), 0));
    let mut remain = 0;

    for _ in 1..k {
        let (Reverse(s), layer) = heap.pop().unwrap();
        remain = s;

        if layer < nums.len() - 1 {
            let e0 = nums[layer];
            let e1 = nums[layer + 1];

            let left = (Reverse(s + e1), layer + 1);
            let right = (Reverse(s + e1 - e0), layer + 1);

            heap.push(left);
            heap.push(right);
        }
    }

    sum - remain
}

fn main() {
    let res = Solution::k_sum(vec![2, 4, -2], 5);
    println!("{res}")
}
