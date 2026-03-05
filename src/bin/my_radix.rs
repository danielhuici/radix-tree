use radix_tree::{Node, Radix};

fn main() {
    let mut tree = Node::<char, bool>::new("", Some(false));

    tree.insert("alligator", true);
    tree.insert("alien", true);
    tree.insert("baloon", true);
    tree.insert("chromodynamic", true);
    tree.insert("romane", true);
    tree.insert("romanus", true);
    tree.insert("romulus", true);
    tree.insert("rubens", true);
    tree.insert("ruber", true);
    tree.insert("rubicon", true);
    tree.insert("rubicundus", true);
    tree.insert("all", true);
    tree.insert("rub", true);
    tree.insert("ba", true);
    tree.insert("你好，世界", true);
    tree.insert("你好", true);
    tree.insert("你", true);

    let node = tree.find("all");
    assert_eq!(node.is_some(), true);
    assert_eq!(node.unwrap().data.unwrap(), true);
    println!("✓ Found 'all' key successfully");

    let node = tree.find("dota2");
    assert_eq!(node.is_none(), true);
    println!("✓ Correctly returned None for 'dota2'");

    println!("All tests passed!");
}
