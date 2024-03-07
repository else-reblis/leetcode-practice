use std::{ops::Add, vec};

struct Solution;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        find_k_or(nums, k as u8)
    }
}

#[derive(Debug)]
struct Binary([u8; 31]);

impl Binary {
    fn zero() -> Self {
        Self([0; 31])
    }

    fn k_or(&self, k: u8) -> i32 {
        let res: [u8; 31] = self.0.map(|x| if x >= k { 1 } else { 0 });
        i32::from(Self(res))
    }
}

impl From<i32> for Binary {
    fn from(mut num: i32) -> Self {
        assert!(num >= 0);

        let mut bits = [0u8; 31];
        for i in 0..31 {
            bits[i] = (num % 2) as u8;
            num /= 2;
        }

        Self(bits)
    }
}

impl Add for Binary {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut output_bits = [0u8; 31];

        for (i, bit) in output_bits.iter_mut().enumerate() {
            *bit = self.0[i] + other.0[i];
        }

        Self(output_bits)
    }
}

impl From<Binary> for i32 {
    fn from(bin: Binary) -> Self {
        let bits = bin.0;
        let mut sum = 0;

        for (i, bit) in bits.iter().enumerate() {
            let i = i as u32;
            let factor = 2i32.pow(i);
            sum += factor * (*bit as i32);
        }

        sum
    }
}

fn find_k_or(nums: Vec<i32>, k: u8) -> i32 {
    let mut sum = Binary::zero();

    for num in nums {
        let bin = Binary::from(num);
        sum = sum + bin;
    }

    sum.k_or(k) as i32
}

pub fn find_k_or_others(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    for i in 0..31i32 {
        let mut cc = 0;
        for v in nums.iter() {
            if (*v & (1 << i)) != 0 {
                cc += 1;
            }
        }
        if cc >= k {
            ans |= 1 << i;
        }
    }
    ans
}
// 唉我是弱智。

fn main() {
    let res = Solution::find_k_or(vec![7, 12, 9, 8, 9, 15], 4);
    assert_eq!(res, 9);
}
