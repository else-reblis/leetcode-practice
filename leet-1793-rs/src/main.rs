struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        maximum_score(nums, k as usize)
    }
}

// pub fn maximum_score(nums: Vec<i32>, k: usize) -> i32 {
//     let k = k as usize;
//     let n = nums.len();
//     let mut dp = vec![vec![0; n - k]; k + 1];
//
//     for i in 0..=k {
//         for j in 0..=n - k - 1 {
//             let max = match (i > 0, j > 0) {
//                 (true, true) => dp[i - 1][j].max(dp[i][j - 1]),
//                 (true, false) => dp[i - 1][j],
//                 (false, true) => dp[i][j - 1],
//                 (false, false) => 0,
//             };
//             let now = range_min_sum(&nums, k - i, k + j);
//             println!("{i}, {j} -> {now}");
//             dp[i][j] = now.max(max);
//         }
//     }
//
//     dp[k][n - k - 1]
// }
//
// fn range_min_sum(nums: &Vec<i32>, left: usize, right: usize) -> i32 {
//     *nums.into_iter().skip(left).take(right - left + 1).min().unwrap_or(&0) * (right - left + 1) as i32
// }

// (k-1, 0) | (k+1, n-1)
// i, j
// dp[i][j] = max(dp[i-1][j], dp[i][j-1], now)


// My ugly AC solution
pub fn maximum_score(nums: Vec<i32>, k: usize) -> i32 {
    let (mut i, mut j) = (k, k);
    let n = nums.len();
    let mut min = nums[k];
    let mut ans = 0;

    let mut flag;

    loop {
        while i > 0 && nums[i - 1] >= min {
            i -= 1;
        }

        while j < n - 1 && nums[j + 1] >= min {
            j += 1;
        }

        let intvl = (j - i + 1) as i32;
        ans = ans.max(intvl * min);

        // dbg!(i, j, min, ans);

        flag = {
            if i == 0 {
                if j == n - 1 {
                    break ans
                }
                false
            } else if j == n - 1 {
                true
            } else {
                nums[i - 1] > nums[j + 1]
            }
        };

        if flag {
            if i > 0 {
                i -= 1;
                min = nums[i];
            }
        } else {
            if j < n - 1 {
                j += 1;
                min = nums[j];
            }
        }
    }
}

// Splendid
pub fn _maximum_score(nums: Vec<i32>, k: usize) -> i32 {
    let (mut min, n) = (nums[k], nums.len());
    let (mut i, mut j) = (k, k);

    let mut res = min;

    for _ in 0..n - 1 {
        if j == n - 1 || i > 0 && nums[i - 1] > nums[j + 1] { // Wow
            i -= 1;
            min = min.min(nums[i]);
        } else {
            j += 1;
            min = min.min(nums[j]);
        }
        res = res.max(min * (j - i + 1) as i32)
    }

    res
}

fn main() {
    let res = Solution::maximum_score(
        vec![
            8182, 1273, 9847, 6230, 52, 1467, 6062, 726, 4852, 4507, 2460, 2041, 500, 1025, 5524,
        ],
        8,
    );
    println!("res = {res}");

    let res = Solution::maximum_score(
        vec![
            5868, 4839, 3608, 4826, 2829, 2871, 3224, 3579, 9860, 6453, 458, 1617, 2201, 2906,
            8334, 8410, 3104, 1385, 5462, 4287, 2960, 5417, 2640, 2287, 6331, 4494, 773, 1178,
            1426, 5271, 3221, 7606, 1091, 9043, 7008, 7218, 6004, 4636, 6253, 5628, 9533, 2204,
            6903, 6912, 9736, 793, 8529, 190, 1302, 4140, 3212, 2589, 2735, 109, 9090, 8351, 332,
            8693, 2461, 6274, 7233, 2689, 7255, 3894, 3855, 1037, 7385, 3874,
        ],
        38,
    );
    println!("res = {res}");
}
