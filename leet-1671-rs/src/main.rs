struct Solution;

// TODO: Too slow. Make it run faster!
// https://leetcode.cn/problems/minimum-number-of-removals-to-make-mountain-array/solutions/2570598/zui-chang-di-zeng-zi-xu-lie-by-leetcode-2ipno/

impl Solution {
    pub fn minimum_mountain_removals(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let seq_lis = lis(nums.clone());
        nums.reverse();
        let rev_lis = lis(nums);

        dbg!(&seq_lis, &rev_lis);

        len as i32
            - seq_lis
                .iter()
                .zip(rev_lis.iter().rev())
                .map(|(&a, &b)| if a <= 1 || b <= 1 { 0 } else {a + b - 1})
                .max()
                .unwrap()
    }
}

pub fn lis(nums: Vec<i32>) -> Vec<i32> {
    let mut dp: Vec<(i32, usize)> = Vec::with_capacity(nums.len());

    for e in nums {
        let longest = dp
            .iter()
            .rev()
            .filter(|&x| x.0 < e)
            .map(|x| x.1)
            .max()
            .unwrap_or(0);
        dp.push((e, longest + 1))
    }

    dp.iter().map(|x| x.1 as i32).collect()
}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut seq = Vec::new();

    for num in nums.into_iter() {
        let idx = seq.partition_point(|&x| x < num);

        if idx == seq.len() {
            seq.push(num);
        } else {
            seq[idx] = num;
        }
    }

    seq.len() as i32
}

fn main() {
    let res = Solution::minimum_mountain_removals(vec![9, 8, 1, 7, 6, 5, 4, 3, 2, 1]);
    println!("res = {:?}", res);
}
