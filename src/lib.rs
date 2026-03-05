#![no_std]
extern crate alloc;

use alloc::borrow::ToOwned;
#[allow(unused_imports)]
// ambiguity between `vec!` macro and `vec` module doesn't sit well with compiler
use alloc::vec;
use alloc::vec::Vec;
use core::mem;

const fn pos<K>(l: &usize, _: &K, _: &Vec<K>) -> usize {
    *l
}

pub trait Vectorable<K>
where
    K: Copy + PartialEq + PartialOrd,
{
    fn to_vec(&self) -> Vec<K>;
}

#[macro_use]
pub mod macros;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<K, V> {
    pub path: Vec<K>, // path from root to node
    pub data: Option<V>,
    pub indices: Vec<K>, // indices of children
    pub nodes: Vec<Node<K, V>>,
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

    pub fn insert_with<F>(
        &mut self,
        path: &mut Vec<K>,
        data: Option<V>,
        force_update: bool,
        pos: F,
    ) -> &mut Self
    where
        F: Fn(&usize, &K, &Vec<K>) -> usize,
    {
        let pl = path.len();
        let sl = self.path.len();

        // empty input path
        if 0 == pl {
            if force_update {
                self.data = data;
            }
            return self;
        }

        // empty node
        if 0 == sl && 0 == self.indices.len() {
            if force_update {
                self.data = data;
            }
            self.path = path.to_owned();
            return self;
        }

        // pl > 0 && sl >= 0
        let max = pl.min(sl);
        let mut i = 0;
        while i < max && path[i] == self.path[i] {
            i += 1;
        }

        if i < sl {
            let child = Node {
                data: self.data.take(),
                path: self.path.split_off(i),
                nodes: mem::replace(&mut self.nodes, Vec::new()),
                indices: mem::replace(&mut self.indices, Vec::new()),
            };
            let c = child.path[0];
            let index = pos(&self.indices.len(), &c, &self.indices);
            self.indices.insert(index, c);
            self.nodes.insert(index, child);

            // self.indices.push(child.path[0]);
            // self.nodes.push(child);
        }

        if i == pl {
            if force_update {
                self.data = data;
            }
            return self;
        }

        self.add_node_with(path, data, i, force_update, pos)
    }

    pub fn add_node_with<F>(
        &mut self,
        path: &mut Vec<K>,
        data: Option<V>,
        i: usize,
        force_update: bool,
        pos: F,
    ) -> &mut Self
    where
        F: Fn(&usize, &K, &Vec<K>) -> usize,
    {
        let l = self.indices.len();
        let c = path[i];
        let mut j = 0;
        while j < l {
            if c == self.indices[j] {
                return self.nodes[j].insert_with(&mut path.split_off(i), data, force_update, pos);
            }
            j += 1;
        }

        let index = pos(&l, &c, &self.indices);
        self.indices.insert(index, c);
        self.nodes.insert(
            index,
            Node {
                data,
                nodes: Vec::new(),
                indices: Vec::new(),
                path: path.split_off(i),
            },
        );

        &mut self.nodes[index]

        // self.indices.push(c);
        // self.nodes.push(Node {
        //     data,
        //     nodes: Vec::new(),
        //     indices: Vec::new(),
        //     path: path.split_off(i),
        // });
        // &mut self.nodes[l]
    }

    pub fn find_with(&self, path: &mut Vec<K>) -> Option<&Self> {
        let pl = path.len();
        let sl = self.path.len();

        // "abc" < "abcde"
        // not found
        if pl < sl {
            return None;
        }

        // "abcde" > "abc" or "abc" == "abc"
        let mut i = 0;
        while i < sl && path[i] == self.path[i] {
            i += 1;
        }

        // "abc" == "abc"
        if pl == sl {
            if i == pl {
                return Some(self);
            }
            // not found
            return None;
        }

        // "abcde" > "abc"
        self.find_node_with(path, i)
    }

    pub fn find_node_with(&self, path: &mut Vec<K>, i: usize) -> Option<&Self> {
        let l = self.indices.len();
        let c = path[i];
        let mut j = 0;
        while j < l {
            if c == self.indices[j] {
                return self.nodes[j].find_with(&mut path.split_off(i));
            }
            j += 1;
        }
        // not found
        None
    }

    #[allow(unused_variables)]
    pub fn remove<P: Vectorable<K>>(&mut self, path: P) {}

    pub fn insert<P: Vectorable<K>>(&mut self, path: P, data: V) -> &mut Self {
        self.insert_with(&mut path.to_vec(), Some(data), true, pos)
    }

    pub fn find<P: Vectorable<K>>(&self, path: P) -> Option<&Self> {
        self.find_with(&mut path.to_vec())
    }

    pub fn add_node<P: Vectorable<K>>(&mut self, path: P, data: V) -> &mut Self {
        self.add_node_with(&mut path.to_vec(), Some(data), 0, true, pos)
    }

    pub fn find_node<P: Vectorable<K>>(&self, path: P) -> Option<&Self> {
        self.find_node_with(&mut path.to_vec(), 0)
    }
}
