use radix_tree::{Node, Radix};

macro_rules! find {
    ($tree:expr, $($path:expr, $data:expr),*,) => {{
        $(
            let node = $tree.find($path);
            assert_eq!(node.is_some(), $data);
            if node.is_some() {
                assert_eq!(node.unwrap().data.unwrap(), $data);
            }
        )*
    }};
}

macro_rules! insert_and_find {
    ($tree:expr, $($path:expr, $data:expr),*,) => {{
        $(
            $tree.insert($path, $data);
            find!($tree, $path, $data,);
        )*
    }};
}

#[test]
fn new_any_type_node() {
    let node = Node::<u8, &str>::new("Hello world!", Some("a"));
    assert_eq!(
        node.path,
        vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33]
    );
    assert_eq!(node.data.unwrap(), "a");

    let node = Node::<u8, &str>::new("Hello 世界！", Some("a"));
    assert_eq!(
        node.path,
        vec![72, 101, 108, 108, 111, 32, 228, 184, 150, 231, 149, 140, 239, 188, 129]
    );
    assert_eq!(node.data.unwrap(), "a");

    let node = Node::<char, &str>::new("Hello 世界！", Some("a"));
    assert_eq!(
        node.path,
        vec!['H', 'e', 'l', 'l', 'o', ' ', '世', '界', '！']
    );
    assert_eq!(node.data.unwrap(), "a");

    let node = Node::<char, u32>::new("你好，世界！", Some(0));
    assert_eq!(node.path, vec!['你', '好', '，', '世', '界', '！']);
    assert_eq!(node.data.unwrap(), 0);

    let node = Node::<u8, u8>::new("abcde", Some(1));
    assert_eq!(node.path, vec![97, 98, 99, 100, 101]);
    assert_eq!(node.data.unwrap(), 1);

    let node = Node::new("abcde".as_bytes().to_vec(), Some(97));
    assert_eq!(node.path, vec![97, 98, 99, 100, 101]);
    assert_eq!(node.data.unwrap(), 97);

    let node = Node::new("abcde".as_bytes(), Some(97));
    assert_eq!(node.path, vec![97, 98, 99, 100, 101]);
    assert_eq!(node.data.unwrap(), 97);
}

#[test]
fn node_insert_and_find() {
    let mut node = Node::<char, bool>::new("你好，世界！", Some(true));
    node.add_node("Rust", true);

    let n1 = node.find_node("Rust");
    let n2 = node.find("你好，世界！Rust");
    assert_eq!(n1, n2);
}

#[test]
fn node_insert_then_return_new_node() {
    let mut tree = Node::<u8, u8>::new("", Some(b' '));

    let a = tree.insert("a", b'a');
    let b = a.add_node("b", b'b');
    let c = b.add_node("c", b'c');
    let d = c.add_node("d", b'd');
    let _ = d.add_node("e", b'e');

    let node = tree.find("a");
    assert_eq!(node.is_some(), true);
    let a = node.unwrap();
    assert_eq!(a.data.unwrap(), b'a');

    let node = a.find_node("b");
    assert_eq!(node.is_some(), true);
    let b = node.unwrap();
    assert_eq!(b.data.unwrap(), b'b');

    let node = b.find_node("c");
    assert_eq!(node.is_some(), true);
    let c = node.unwrap();
    assert_eq!(c.data.unwrap(), b'c');

    let node = c.find_node("d");
    assert_eq!(node.is_some(), true);
    let d = node.unwrap();
    assert_eq!(d.data.unwrap(), b'd');

    let node = d.find_node("e");
    assert_eq!(node.is_some(), true);
    let e = node.unwrap();
    assert_eq!(e.data.unwrap(), b'e');

    let node = a.find("abcde");
    assert_eq!(node.is_some(), true);
    assert_eq!(node.unwrap().data.unwrap(), b'e');

    let node = tree.find("abcdef");
    assert_eq!(node.is_some(), false);

    let node = tree.find("b");
    assert_eq!(node.is_some(), false);
}

#[test]
fn new_tree() {
    let mut tree = Node::<char, bool>::new("", Some(false));

    insert_and_find!(
        tree,
        "alligator",
        true,
        "alien",
        true,
        "baloon",
        true,
        "chromodynamic",
        true,
        "romane",
        true,
        "romanus",
        true,
        "romulus",
        true,
        "rubens",
        true,
        "ruber",
        true,
        "rubicon",
        true,
        "rubicundus",
        true,
        "all",
        true,
        "rub",
        true,
        "ba",
        true,
        "你好，世界",
        true,
        "你好",
        true,
        "你",
        true,
    );

    find!(
        tree, "rpxxx", false, "chro", false, "chromz", false, "zorro", false, "ro", false, "zo",
        false,
    );

    let node = tree.find("");
    assert_eq!(node.is_some(), true);
    assert_eq!(node.unwrap().data, None);

    tree.insert("", false);
    let node = tree.find("");
    assert_eq!(node.is_some(), true);
    assert_eq!(node.unwrap().data.unwrap(), false);

    let node = tree.find("all");
    assert_eq!(node.is_some(), true);
    assert_eq!(node.unwrap().data.unwrap(), true);

    let node = tree.find("dota2");
    assert_eq!(node.is_none(), true);

    let node = tree.find("你");
    assert_eq!(node.is_some(), true);
    assert_eq!(node.unwrap().data.unwrap(), true);

    let node = tree.find("你好");
    assert_eq!(node.is_some(), true);
    assert_eq!(node.unwrap().data.unwrap(), true);

    let node = tree.find("语言");
    assert_eq!(node.is_some(), false);

    let node = tree.find("你好，世界");
    assert_eq!(node.is_some(), true);

    let node = tree.find("你好，世界 Rust");
    assert_eq!(node.is_some(), false);
}

#[test]
fn clone_node() {
    let mut node = Node::<char, bool>::new("", Some(false));
    let mut node2 = node.clone();
    assert_eq!(node, node2);

    node.add_node("/", true);
    node2.add_node("/", true);
    assert_eq!(node, node2);

    #[derive(Debug, Clone, PartialEq)]
    struct NodeMetadata {
        is_key: bool,
        params: Option<Vec<&'static str>>,
    }

    let mut node = Node::<char, NodeMetadata>::new(
        "/",
        Some(NodeMetadata {
            is_key: false,
            params: Some(vec![]),
        }),
    );
    let mut node2 = node.clone();
    assert_eq!(node, node2);

    node.add_node(
        "users",
        NodeMetadata {
            is_key: true,
            params: Some(vec!["tree"]),
        },
    );
    node2.add_node(
        "users",
        NodeMetadata {
            is_key: true,
            params: Some(vec!["tree"]),
        },
    );
    assert_eq!(node, node2);
}
