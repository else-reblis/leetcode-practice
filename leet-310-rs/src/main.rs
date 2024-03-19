use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

pub fn get_adj_mat(n: usize, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut mat = vec![vec![-1; n]; n];
    let edges: HashSet<_> = edges.into_iter().map(|e| (e[0], e[1])).collect();


    todo!()
}

// pub fn distance(mat: &mut Vec<Vec<i32>>, edges: &HashSet<(usize, usize)>, i: usize, j: usize) -> {
//     if edges.contains(&(i, j)) || edges.contains(&(j, i)) {
//         mat[i][j] = 1;
//         mat[j][i] = 1;
//     } else if mat[i][j] != -1 {
//         mat[i][j]
//     } else {
//         let distance = 0;
//         for k in 0..n {
//             todo!()
//         }
//     }
// }

fn main() {
    println!("Hello, world!");
}
