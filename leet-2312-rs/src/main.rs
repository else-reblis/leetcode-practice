struct Solution;

macro_rules! s {
    ($n:expr) => {
        $n as usize
    };
}

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        selling_wood(m, n, prices)
    }
}

pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
    let mut dp = vec![vec![0i64; s!(1 + n)]; s!(1 + m)];

    for tuple in prices {
        let (x, y) = (s!(tuple[0]), s!(tuple[1]));
        dp[x][y] = tuple[2] as i64;
    }

    for x in 1..=m {
        for y in 1..=n {
            let (x, y) = (s!(x), s!(y));
            let ln = (1..x).map(|i| dp[x - i][y] + dp[i][y]).max().unwrap_or(0);
            let co = (1..y).map(|j| dp[x][y - j] + dp[x][j]).max().unwrap_or(0);
            dp[x][y] = dp[x][y].max(ln.max(co));
        }
    }

    println!("{:?}", dp);

    dp[s!(m)][s!(n)]
}

fn main() {
    let res = Solution::selling_wood(3, 5, vec![vec![1, 4, 2], vec![2, 2, 7], vec![2, 1, 3]]);
    println!("{res}");
}
