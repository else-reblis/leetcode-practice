use std::collections::BinaryHeap;

use rand::prelude::*;

struct Solution;

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        k_sum(nums, k)
    }
}

const N: usize = 8;

#[derive(Debug)]
pub struct FixedHeap<T> {
    inner: [T; N],
    size: usize,
}

pub trait Minimum {
    fn minimum() -> Self;
}

impl<T> FixedHeap<T>
where
    T: Copy + PartialOrd + Minimum + std::fmt::Debug,
{
    fn empty() -> Self {
        let inner = [T::minimum(); N];

        FixedHeap { inner, size: 0 }
    }

    fn insert(&mut self, t: T) {
        let mut index = self.size + 1;

        if index < N - 1 {
            self.inner[index] = t;
            self.size += 1;
        } else {
            for i in (N / 2..N).rev() {
                if self.inner[i] < t {
                    self.inner[i] = t;
                    index = i;
                    break;
                }
            }
        }

        self.swim(index);
    }

    fn swim(&mut self, mut index: usize) {
        while index > 1 && self.inner[index / 2] < self.inner[index] {
            let child = self.inner[index];
            self.inner[index] = self.inner[index / 2];
            self.inner[index / 2] = child;
            index /= 2;
        }
    }

    fn peek(&self) -> T {
        self.inner[1]
    }

    fn pop(&mut self) -> T {
        let mut index = 1;
        let popcorn = self.inner[1];

        while index < N / 2 {
            let left = self.inner[index * 2];
            let right = self.inner[index * 2 + 1];

            if left > right {
                self.inner[index * 2] = self.inner[index];
                index = index * 2
            } else {
                index = index * 2 + 1
            }
        }

        if index < N {
            self.inner[index] = T::minimum();
        }
        popcorn
    }
}

impl Minimum for i32 {
    fn minimum() -> i32 {
        i32::MIN
    }
}

pub fn insertion_test_1() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (0..30).collect();

    let mut heap = FixedHeap::empty();

    nums.shuffle(&mut rng);

    for num in nums {
        println!("INSERT          : {num}");
        println!("BEFORE INSERTION: {:?}", heap);
        heap.insert(num);
        println!("AFTER INSERTION : {:?}", heap);
    }

    dbg!(&heap);

    for _ in 0..16 {
        let res = heap.pop();
        println!("{res}")
    }
}

#[derive(Copy, Clone)]
struct Node(i64, usize);

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let pos_sum: i64 = nums.iter().map(|x| *x as i64).filter(|x| *x > 0).sum();
    let mut nums = nums.into_iter().map(|x| x.abs()).collect::<Vec<_>>();
    nums.sort();

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let init = Node(nums[0] as i64, 0);

    heap.push(init);
    let mut remain = 0;

    for _ in 1..k {
        let node = heap.pop().unwrap();
        let layer = node.1;
        remain = node.0;

        if layer < n - 1 {
            let e0 = nums[layer] as i64;
            let e1 = nums[layer + 1] as i64;

            let left = Node(remain + e1, layer + 1);
            let right = Node(remain - e0 + e1, layer + 1);

            heap.push(left);
            heap.push(right);
        }

        println!("remain = {remain}");
    }

    pos_sum - remain
}

fn main() {
    let res = Solution::k_sum(vec![2, 4, -2], 5);
    println!("{res}")
}
