#![no_std]
extern crate alloc;

use alloc::borrow::ToOwned;
#[allow(unused_imports)]
// ambiguity between `vec!` macro and `vec` module doesn't sit well with compiler
use alloc::vec;
use alloc::vec::Vec;
use core::mem;

pub trait Vectorable<K>
where
    K: Copy + PartialEq + PartialOrd,
{
    fn to_vec(&self) -> Vec<K>;
}

#[macro_use]
pub mod macros;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "K: serde::Serialize, V: serde::Serialize",
        deserialize = "K: serde::Deserialize<'de>, V: serde::Deserialize<'de>"
    ))
)]
pub struct Node<K, V> {
    pub path: Vec<K>,
    pub data: Option<V>,
    pub indices: Vec<K>,        // indices of children nodes
    pub nodes: Vec<Node<K, V>>, // childrens of the node
}

impl<K, V> Node<K, V>
where
    K: Copy + PartialEq + PartialOrd,
    V: Clone,
{
    pub fn new<P: Vectorable<K>>(path: P, data: Option<V>) -> Self {
        Node {
            data,
            path: path.to_vec(),
            nodes: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn insert_with(
        &mut self,
        new_key: &mut Vec<K>,
        data: Option<V>,
        force_update: bool,
    ) -> &mut Self {
        let new_key_len = new_key.len();
        let node_path_len = self.path.len();

        // Empty key
        if new_key_len == 0 {
            if force_update {
                self.data = data;
            }
            return self;
        }

        // The radix tree is empty
        if node_path_len == 0 && self.indices.is_empty() {
            if force_update {
                self.data = data;
            }
            self.path = new_key.to_owned();
            return self;
        }

        // Find the common prefix length
        let prefix_len = new_key
            .iter()
            .zip(&self.path)
            .take_while(|(a, b)| a == b)
            .count();

        // Split the current node if the new key diverges early
        if prefix_len < node_path_len {
            let child = Node {
                data: self.data.take(),
                path: self.path.split_off(prefix_len),
                nodes: mem::take(&mut self.nodes),
                indices: mem::take(&mut self.indices),
            };
            self.indices = vec![child.path[0]];
            self.nodes = vec![child];
        }

        // The new key matches the current node's path
        if prefix_len == new_key_len {
            if force_update {
                self.data = data;
            }
            return self;
        }

        // Else, delegate to a child, or create a new leaf
        let branch_key = new_key[prefix_len];

        // Check if a child already handles this path
        if let Some(child_idx) = self.indices.iter().position(|&x| x == branch_key) {
            let mut remaining_path = new_key.split_off(prefix_len);
            return self.nodes[child_idx].insert_with(&mut remaining_path, data, force_update);
        }

        // In case we didn't find a matching child, we create a new leaf node
        self.indices.push(branch_key);
        self.nodes.push(Node {
            data,
            nodes: Vec::new(),
            indices: Vec::new(),
            path: new_key.split_off(prefix_len), // remaining path
        });

        // Return a mutable reference to the newly pushed leaf
        self.nodes.last_mut().unwrap()
    }

    pub fn find_with(&self, query: &mut Vec<K>) -> Option<&Self> {
        let query_len = query.len();
        let node_path_len = self.path.len();

        // The search path is shorter than the node's path. It cannot be here
        if query_len < node_path_len {
            return None;
        }

        // Find where the query diverges from the key
        let prefix_len = query
            .iter()
            .zip(&self.path)
            .take_while(|(a, b)| a == b)
            .count();

        // If the match didn't cover the entire node's path, it diverges here
        if prefix_len < node_path_len {
            return None;
        }

        // Query and node path both lengths matches. FOUND!
        if query_len == node_path_len {
            return Some(self);
        }

        // Query is longer, so we need to look into the children
        let branch_char = query[prefix_len];
        if let Some(child_idx) = self.indices.iter().position(|&x| x == branch_char) {
            let mut remaining_path = query.split_off(prefix_len);
            return self.nodes[child_idx].find_with(&mut remaining_path);
        }

        // No matching child found
        None
    }

    #[allow(unused_variables)]
    pub fn remove<P: Vectorable<K>>(&mut self, path: P) {}

    pub fn insert<P: Vectorable<K>>(&mut self, path: P, data: V) -> &mut Self {
        self.insert_with(&mut path.to_vec(), Some(data), true)
    }
    pub fn find<P: Vectorable<K>>(&self, path: P) -> Option<&Self> {
        self.find_with(&mut path.to_vec())
    }
}
