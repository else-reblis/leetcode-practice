// use std::collections::VecDeque;

struct Solution;

// impl Solution {
//     pub fn max_array_value(nums: Vec<i32>) -> i64 {
//         let mut nums = nums.into_iter().map(|e| e as i64).rev().collect::<Vec<_>>();
//         let mut len = nums.len();
//
//         while {
//             nums = max_array_one_iter(nums);
//             nums.len() < len
//         } {
//             len = nums.len();
//         }
//
//         nums.into_iter().max().unwrap()
//     }
// }

// pub fn max_array_one_iter(nums: Vec<i64>) -> Vec<i64> {
//     let mut queue: VecDeque<_> = nums.into_iter().collect();
//     let mut res = vec![];
//     let mut e = queue.pop_front().unwrap();
//
//     while let Some(first) = queue.pop_front() {
//         if e >= first {
//             e += first
//         } else {
//             res.push(e);
//             e = first;
//         }
//     }
//
//     res.push(e);
//     res
// }

// pub fn max_array_value(mut nums: Vec<i32>) -> i64 {
//     let mut e = nums.pop().unwrap() as i64;
//
//     while let Some(first) = nums.pop() {
//         let first = first as i64;
//         if e >= first {
//             e += first
//         } else {
//             e = first;
//         }
//     }
//
//     e
// }

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let mut max = i64::MIN;

        nums.into_iter().rev().for_each(|e| {
            let e = e as i64;
            if max >= e {
                max += e
            } else {
                max = e
            }
        });

        max
    }
}
fn main() {
    let res = Solution::max_array_value(vec![5, 3, 3]);
    println!("{:?}", res);
}
