#[allow(unused)]
struct NumArray {
    sum_array: Vec<i64>
}

impl NumArray {
    #[allow(unused)]
    fn new(mut nums: Vec<i32>) -> Self {
        let mut sum: i64 = 0;

        nums.insert(0, 0);
        let sums = nums.into_iter().map(|e| {sum += e as i64 ; sum}).collect();

        Self {
            sum_array: sums,
        }
    }

    #[allow(unused)]
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        (self.sum_array[right as usize + 1] - self.sum_array[left as usize]) as i32
    }
}

fn main() {
}
