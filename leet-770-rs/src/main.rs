use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String> {
        todo!()
    }
}

struct BasicCalc {
    expression: AST,
    eval_map: HashMap<String, i32>
}

struct AST {
    node: ASTNode,
}

struct ASTNode {
}

impl AST {
    fn normalize(&mut self) {
    }

    fn parse() -> Self {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
