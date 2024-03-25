struct Solution;

const MOD: i64 = 1000000007;

impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        let res = result();
        res[p as usize]
    }
}

const fn result() -> [i32; 61] {
    let mut res = [0; 61];
    let mut idx = 0;

    while idx < 61 {
        res[idx] = min_non_zero_product(idx as i32);
        idx += 1;
    }

    res
}

const fn min_non_zero_product(p: i32) -> i32 {
    match p {
        1 => 1,
        2 => 6,
        p => {
            let pow = 2i64.pow((p - 1) as u32);
            let base = pow_mod(2, p as u64, MOD) - 1;
            let res = base * pow_mod(base - 1, pow as u64 - 1, MOD);
            // dbg!(pow, base, res);

            (res % MOD) as i32
        }
    }
}


const fn pow_mod(mut a: i64, mut b: u64, m: i64) -> i64 {
    let mut res: i128 = 1;

    while b > 0 {
        if b % 2 == 1 {
            res = (res * a as i128) % m as i128;
        }

        a = ((a as i128 * a as i128) % m as i128) as i64;
        b /= 2;
    }

    res as i64
}


fn main() {
    let res = Solution::min_non_zero_product(31);
    println!("res = {res}");
}
