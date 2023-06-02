use bt::BinaryTree;

mod bt;
mod bt_concrete;

fn main() {
    println!("---------------------- INT BINARY TREES ----------------------");
    let trees_i32: Vec<&BinaryTree<i32>> = vec![
        &(*bt_concrete::BT_EMPTY_I32),
        &(*bt_concrete::BT_SINGLE_I32),
        &(*bt_concrete::BT_LEFT_I32),
        &(*bt_concrete::BT_LR_I32),
        &(*bt_concrete::BT_FULL_I32),
        &(*bt_concrete::BT_FULL_IMB_I32),
    ];

    for (i, tree) in trees_i32.iter().enumerate() {
        let tree_name: &str = bt_concrete::TREE_NAMES[i];
        println!("{}", tree_name);
        tree.display();
        println!("Size: {}, Height: {}", tree.size(), tree.height());
        let mut s: String = String::new();
        for v in 5..14 {
            if tree.contains(&v) {
                s += &v.to_string();
                s += &String::from(", ");
            }
        }
        if s.len() > 2 {
            s = s[..s.len()-2].to_string();
        }
        println!("Tree contains: {}", s);
        println!("Traversals:");
        println!("\t{}", tree.traverse(bt::Order::Pre));
        println!("\t{}", tree.traverse(bt::Order::In));
        println!("\t{}", tree.traverse(bt::Order::Post));
    }
    
    println!("---------------------- STR BINARY TREES ----------------------");
    let trees_str: Vec<&BinaryTree<String>> = vec![
        &(*bt_concrete::BT_EMPTY_STR),
        &(*bt_concrete::BT_SINGLE_STR),
        &(*bt_concrete::BT_LEFT_STR),
        &(*bt_concrete::BT_LR_STR),
        &(*bt_concrete::BT_FULL_STR),
        &(*bt_concrete::BT_FULL_IMB_STR),
    ];

    for (i, tree) in trees_str.iter().enumerate() {
        let tree_name: &str = bt_concrete::TREE_NAMES[i];
        println!("{}", tree_name);
        tree.display();
        println!("Size: {}, Height: {}", tree.size(), tree.height());
        let mut s: String = String::new();
        for v in 'h'..'q' {
            if tree.contains(&v.to_string()) {
                s += &v.to_string();
                s += &String::from(", ");
            }
        }
        if s.len() > 2 {
            s = s[..s.len()-2].to_string();
        }
        println!("Tree contains: {}", s);
        println!("Traversals:");
        println!("\t{}", tree.traverse(bt::Order::Pre));
        println!("\t{}", tree.traverse(bt::Order::In));
        println!("\t{}", tree.traverse(bt::Order::Post));
    }

    println!("--------------------------- INSERTS ---------------------------");
    let mut tree: bt::BinaryTree<i32> = bt::BinaryTree::None;
    println!("Before:");
    tree.display();
    for v in [4, 2, 3, 9, 1, 2, 6, 7, 0, 9, 1] {
        println!("Inserting {}", v);
        tree.insert(v);
    }
    println!("Now, the tree looks like:");
    tree.display();
    println!("Traversals:");
    println!("\t{}", tree.traverse(bt::Order::Pre));
    println!("\t{}", tree.traverse(bt::Order::In));
    println!("\t{}", tree.traverse(bt::Order::Post));

    let mut tree: bt::BinaryTree<String> = bt::BinaryTree::None;
    println!("Before:");
    tree.display();
    for v in ['g', 'e', 'l', 'p', 'a', 'x', 'q', 'd', 'f', 'c', 'g'] {
        println!("Inserting {}", v);
        tree.insert(v.to_string());
    }
    println!("Now, the tree looks like:");
    tree.display();
    println!("Traversals:");
    println!("\t{}", tree.traverse(bt::Order::Pre));
    println!("\t{}", tree.traverse(bt::Order::In));
    println!("\t{}", tree.traverse(bt::Order::Post));

}
