use crate::bt;

pub static TREE_NAMES: [&str; 6] = [
    "Empty Binary Tree:",
    "Single Node:",
    "Left Node Only:",
    "Both Nodes:",
    "Full:",
    "Full Imbalanced:",
];

lazy_static::lazy_static!{
    // INTEGER BINARY TREES
    pub static ref BT_EMPTY_I32: bt::BinaryTree<i32> = bt::BinaryTree::None;
    pub static ref BT_SINGLE_I32: bt::BinaryTree<i32> = bt::BinaryTree::Some(bt::BTNode{
        value: 10,
        left: None,
        right: None,
    });
    pub static ref BT_LEFT_I32: bt::BinaryTree<i32> = bt::BinaryTree::Some(bt::BTNode{
        value: 10,
        left: Some(Box::new(bt::BTNode{
            value: 9,
            left: None,
            right: None,
        })),
        right: None,
    });
    pub static ref BT_LR_I32: bt::BinaryTree<i32> = bt::BinaryTree::Some(bt::BTNode{
        value: 10,
        left: Some(Box::new(bt::BTNode{
            value: 9,
            left: None,
            right: None,
        })),
        right: Some(Box::new(bt::BTNode{
            value: 11,
            left: None,
            right: None,
        })),
    });
    pub static ref BT_FULL_I32: bt::BinaryTree<i32> = bt::BinaryTree::Some(bt::BTNode{
        value: 10,
        left: Some(Box::new(bt::BTNode{
            value: 8,
            left: Some(Box::new(bt::BTNode{
                value: 7,
                left: None,
                right: None,
            })),
            right: Some(Box::new(bt::BTNode{
                value: 9,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(bt::BTNode{
            value: 12,
            left: Some(Box::new(bt::BTNode{
                value: 11,
                left: None,
                right: None,
            })),
            right: Some(Box::new(bt::BTNode{
                value: 13,
                left: None,
                right: None,
            })),
        })),
    });
    pub static ref BT_FULL_IMB_I32: bt::BinaryTree<i32> = bt::BinaryTree::Some(bt::BTNode{
        value: 10,
        left: Some(Box::new(bt::BTNode{
            value: 8,
            left: Some(Box::new(bt::BTNode{
                value: 7,
                left: Some(Box::new(bt::BTNode{
                    value: 6,
                    left: Some(Box::new(bt::BTNode{
                        value: 5,
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
                right: None,
            })),
            right: Some(Box::new(bt::BTNode{
                value: 9,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(bt::BTNode{
            value: 12,
            left: Some(Box::new(bt::BTNode{
                value: 11,
                left: None,
                right: None,
            })),
            right: Some(Box::new(bt::BTNode{
                value: 13,
                left: None,
                right: None,
            })),
        })),
    });

    // STRING BINARY TREES
    pub static ref BT_EMPTY_STR: bt::BinaryTree<String> = bt::BinaryTree::None;
    pub static ref BT_SINGLE_STR: bt::BinaryTree<String> = bt::BinaryTree::Some(bt::BTNode{
        value: String::from("m"),
        left: None,
        right: None,
    });
    pub static ref BT_LEFT_STR: bt::BinaryTree<String> = bt::BinaryTree::Some(bt::BTNode{
        value: String::from("m"),
        left: Some(Box::new(bt::BTNode{
            value: String::from("l"),
            left: None,
            right: None,
        })),
        right: None,
    });
    pub static ref BT_LR_STR: bt::BinaryTree<String> = bt::BinaryTree::Some(bt::BTNode{
        value: String::from("m"),
        left: Some(Box::new(bt::BTNode{
            value: String::from("l"),
            left: None,
            right: None,
        })),
        right: Some(Box::new(bt::BTNode{
            value: String::from("n"),
            left: None,
            right: None,
        })),
    });
    pub static ref BT_FULL_STR: bt::BinaryTree<String> = bt::BinaryTree::Some(bt::BTNode{
        value: String::from("m"),
        left: Some(Box::new(bt::BTNode{
            value: String::from("k"),
            left: Some(Box::new(bt::BTNode{
                value: String::from("j"),
                left: None,
                right: None,
            })),
            right: Some(Box::new(bt::BTNode{
                value: String::from("l"),
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(bt::BTNode{
            value: String::from("o"),
            left: Some(Box::new(bt::BTNode{
                value: String::from("n"),
                left: None,
                right: None,
            })),
            right: Some(Box::new(bt::BTNode{
                value: String::from("p"),
                left: None,
                right: None,
            })),
        })),
    });
    pub static ref BT_FULL_IMB_STR: bt::BinaryTree<String> = bt::BinaryTree::Some(bt::BTNode{
        value: String::from("m"),
        left: Some(Box::new(bt::BTNode{
            value: String::from("k"),
            left: Some(Box::new(bt::BTNode{
                value: String::from("j"),
                left: Some(Box::new(bt::BTNode{
                    value: String::from("i"),
                    left: Some(Box::new(bt::BTNode{
                        value: String::from("h"),
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
                right: None,
            })),
            right: Some(Box::new(bt::BTNode{
                value: String::from("l"),
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(bt::BTNode{
            value: String::from("o"),
            left: Some(Box::new(bt::BTNode{
                value: String::from("n"),
                left: None,
                right: None,
            })),
            right: Some(Box::new(bt::BTNode{
                value: String::from("p"),
                left: None,
                right: None,
            })),
        })),
    });
 
    
    /*
    pub static ref trees: Vec<&bt::BinaryTree<i32>> = vec![
        &(*bt_empty),
        &(*bt_single_node),
        &(*bt_left_only),
        &(*bt_left_right),
        &(*bt_full),
        &(*bt_full_imbalanced)
    ];
    */
}