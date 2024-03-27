use std::{cmp::Reverse, collections::BinaryHeap};

struct Graph {
    inner: Vec<Vec<(i32, i32)>>,
    n_node: usize,
}

#[allow(unused)]
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut inner = vec![vec![]; n as usize];

        for edge in edges {
            Self::add_edge_inner(&mut inner, edge)
        }

        Self {
            inner,
            n_node: n as usize,
        }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        Self::add_edge_inner(&mut self.inner, edge)
    }

    fn shortest_path(&self, from: i32, to: i32) -> i32 {
        let mut dist = vec![i32::MAX; self.n_node];
        dist[from as usize] = 0;

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), from));

        while let Some((Reverse(dist_from), node_from)) = heap.pop() {
            if node_from == to {
                return dist_from;
            }

            if dist_from > dist[node_from as usize] {
                continue;
            }

            for &(node_to, dist_to) in &self.inner[node_from as usize] {
                let new_dist_to = dist_from + dist_to;
                if new_dist_to < dist[node_to as usize] {
                    dist[node_to as usize] = new_dist_to;
                    heap.push((Reverse(new_dist_to), node_to))
                }
            }
        }

        -1
    }

    fn add_edge_inner(inner: &mut Vec<Vec<(i32, i32)>>, edge: Vec<i32>) {
        let (from, to, weight) = (edge[0], edge[1], edge[2]);
        inner[from as usize].push((to, weight))
    }
}


fn main() {
    println!("Hello, world!");
}
