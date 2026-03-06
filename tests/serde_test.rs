use radix_tree::Node;

#[test]
#[cfg(feature = "serde")]
fn test_node_serde() {
    let mut tree = Node::<char, i32>::new("root", Some(0));
    tree.insert("child", 1);
    tree.insert("other", 2);

    let serialized = serde_json::to_string(&tree).expect("Failed to serialize");
    println!("Serialized: {}", serialized);

    let deserialized: Node<char, i32> =
        serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(tree, deserialized);
    assert_eq!(deserialized.find("root").unwrap().data, Some(0));
    assert_eq!(deserialized.find("child").unwrap().data, Some(1));
}

#[test]
#[cfg(feature = "serde")]
fn test_node_serde_u8() {
    let mut tree = Node::<u8, String>::new("root", Some("data".to_string()));
    tree.insert("a", "b".to_string());

    let serialized = serde_json::to_string(&tree).expect("Failed to serialize");
    let deserialized: Node<u8, String> =
        serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(tree, deserialized);
}
