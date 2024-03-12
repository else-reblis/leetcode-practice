use std::{cell::RefCell, collections::HashSet, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl TreeNode {
    pub fn nodes(&self, node_set: &mut HashSet<i32>, val: i32) {
        node_set.insert(val);

        self.left.iter().for_each(|node| {
            let node = &node.borrow();
            let val = (val + 1) * 2 - 1;
            node.nodes(node_set, val);
        });

        self.right.iter().for_each(|node| {
            let node = &node.borrow();
            let val = (val + 1) * 2;
            node.nodes(node_set, val);
        });
    }
}

#[derive(Debug)]
struct FindElements {
    nodes: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = HashSet::new();
        let root = root.unwrap();
        let root = root.as_ref().borrow();
        root.nodes(&mut nodes, 0);

        FindElements { nodes }
    }

    fn find(&self, target: i32) -> bool {
        self.nodes.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

fn main() {
    let tree_node = TreeNode {
        val: -1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: None,
        }))),
        right: None,
    };

    let fe = FindElements::new(Some(Rc::new(RefCell::new(tree_node))));
    println!("find element 1 = {}", fe.find(1));
    println!("find element 14 = {}", fe.find(14));
    println!("find element 5 = {}", fe.find(5));
}
