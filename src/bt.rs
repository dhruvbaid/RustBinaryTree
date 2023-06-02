use std::{cmp, string::ToString, cmp::PartialEq, cmp::PartialOrd};

pub struct BTNode<T> {
    pub value: T,
    pub left: Option<Box<BTNode<T>>>,
    pub right: Option<Box<BTNode<T>>>,
}

pub enum BinaryTree<T> {
    Some(BTNode<T>),
    None,
}

pub enum Order {
    Pre,
    In,
    Post,
}

impl<T: ToString + PartialEq + PartialOrd> BinaryTree<T> {
    pub fn display(&self) {
        match self {
            BinaryTree::Some(node) => {
                println!("{}", node.display(0));
            },
            BinaryTree::None => {
                println!("Empty Binary Tree!");
            }
        }
    }

    pub fn size(&self) -> i32 {
        match self {
            BinaryTree::Some(node) => {
                node.size()
            },
            BinaryTree::None => {
                0
            }
        }
    }

    pub fn height(&self) -> i32 {
        match self {
            BinaryTree::Some(node) => {
                node.height()
            },
            BinaryTree::None => {
                0
            }
        }
    }

    pub fn contains(&self, i: &T) -> bool {
        match self {
            BinaryTree::Some(node) => {
                node.contains(i)
            },
            BinaryTree::None => {
                false
            }
        }
    }

    pub fn insert(&mut self, val: T) {
        if !self.contains(&val) {
            match self {
                BinaryTree::Some(node) => {
                    node.insert(val)
                },
                BinaryTree::None => {
                    *self = BinaryTree::Some(BTNode{
                        value: val,
                        left: None,
                        right: None,
                    })
                }
            }
        }
    }

    pub fn traverse(&self, order: Order) -> String {
        match self {
            BinaryTree::None => {
                String::from("Empty Binary Tree!")
            },
            BinaryTree::Some(node) => {
                let mut res: String = String::new();
                for v in node.traverse(&order) {
                    res += &v.to_string();
                    res += ", ";
                }
                if res.len() > 2 {
                    res = res[..res.len() - 2].to_string();
                }
                res
            }
        }
    }
}

impl<T: ToString + PartialEq + PartialOrd> BTNode<T> {
    fn display(&self, tabs: i32) -> String {
        match (&self.left, &self.right) {
            (Some(l_node), Some(r_node)) => {
                let mut str_rep: String = String::new();
                str_rep += &((*r_node).display(tabs + 1));
                str_rep += &String::from("\n");
                for _i in 0..tabs {
                    str_rep += &String::from("\t");
                }
                str_rep += &(self.value.to_string());
                str_rep += &String::from("\n");
                str_rep += &((*l_node).display(tabs + 1));
                str_rep
            },
            (Some(l_node), None) => {
                let mut str_rep: String = String::new();
                for _i in 0..tabs {
                    str_rep += &String::from("\t");
                }
                str_rep += &(self.value.to_string());
                str_rep += &String::from("\n");
                str_rep += &((*l_node).display(tabs + 1));
                str_rep
            },
            (None, Some(r_node)) => {
                let mut str_rep: String = String::new();
                str_rep += &((*r_node).display(tabs + 1));
                str_rep += &String::from("\n");
                for _i in 0..tabs {
                    str_rep += &String::from("\t");
                }
                str_rep += &(self.value.to_string());
                str_rep
            },
            (None, None) => {
                let mut str_rep: String = String::new();
                for _i in 0..tabs {
                    str_rep += &String::from("\t");
                }
                str_rep += &(self.value.to_string());
                str_rep
            },
        }
    }

    fn size(&self) -> i32 {
        match (&self.left, &self.right) {
            (Some(l_node), Some(r_node)) => {
                1 + (*l_node).size() + (*r_node).size()
            },
            (Some(l_node), None) => {1 + (*l_node).size()},
            (None, Some(r_node)) => {1 + (*r_node).size()},
            (None, None) => {1},
        }
    }

    fn height(&self) -> i32 {
        match (&self.left, &self.right) {
            (Some(l_node), Some(r_node)) => {
                1 + cmp::max((*l_node).height(), (*r_node).height())
            },
            (Some(l_node), None) => {1 + (*l_node).height()},
            (None, Some(r_node)) => {1 + (*r_node).height()},
            (None, None) => {1},
        }
    }

    fn contains(&self, i: &T) -> bool {
        match (&self.left, &self.right) {
            (Some(l_node), Some(r_node)) => {
                self.value == *i || (*l_node).contains(i) || (*r_node).contains(i)
            },
            (Some(l_node), None) => {
                self.value == *i || (*l_node).contains(i)
            },
            (None, Some(r_node)) => {
                self.value == *i || (*r_node).contains(i)
            },
            (None, None) => {self.value == *i},
        }
    }

    fn insert(&mut self, val: T) {
        match (&mut self.left, &mut self.right) {
            (Some(l_node), Some(r_node)) => {
                if val < self.value {
                    l_node.insert(val);
                } else {
                    r_node.insert(val);
                }
            },
            (Some(l_node), None) => {
                if val < self.value {
                    l_node.insert(val);
                } else {
                    self.right = Some(Box::new(BTNode{
                        value: val,
                        left: None,
                        right: None,
                    }));
                }
            },
            (None, Some(r_node)) => {
                if val < self.value {
                    self.left = Some(Box::new(BTNode{
                        value: val,
                        left: None,
                        right: None,
                    }));
                } else {
                    r_node.insert(val);
                }
            },
            (None, None) => {
                if val < self.value {
                    self.left = Some(Box::new(BTNode{
                        value: val,
                        left: None,
                        right: None,
                    }));
                } else {
                    self.right = Some(Box::new(BTNode{
                        value: val,
                        left: None,
                        right: None,
                    }));
                }
            },
        }
    }

    fn traverse(&self, order: &Order) -> Vec<&T> {
        let mut res: Vec<&T> = Vec::new();
        match (&self.left, &self.right) {
            (Some(l_node), Some(r_node)) => {
                match order {
                    Order::Pre => {
                        res.push(&self.value);
                        res.append(&mut l_node.traverse(order));
                        res.append(&mut r_node.traverse(order));
                    },
                    Order::In => {
                        res.append(&mut l_node.traverse(order));
                        res.push(&self.value);
                        res.append(&mut r_node.traverse(order));
                    },
                    Order::Post => {
                        res.append(&mut l_node.traverse(order));
                        res.append(&mut r_node.traverse(order));
                        res.push(&self.value);
                    },
                }
            },
            (Some(l_node), None) => {
                match order {
                    Order::Pre => {
                        res.push(&self.value);
                        res.append(&mut l_node.traverse(order));
                    },
                    _ => {
                        res.append(&mut l_node.traverse(order));
                        res.push(&self.value);
                    },
                }
            },
            (None, Some(r_node)) => {
                match order {
                    Order::Post => {
                        res.append(&mut r_node.traverse(order));
                        res.push(&self.value);
                    },
                    _ => {
                        res.push(&self.value);
                        res.append(&mut r_node.traverse(order));
                    },
                }
            },
            (None, None) => {
                res.push(&self.value);
            },
        }
        res
    }

}