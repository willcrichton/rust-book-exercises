//! P1: Binary search tree
//!
//! In this file, you will implement methods for an enum-based data structure.
//! Specifically, the BST type represents a binary search tree. In a BST,
//! for a given node T, it must satisfy the invariant that the left subtree
//! of T must contain nodes smaller than T, and the right subtree contains nodes
//! larger than T.
//!
//! For example, the following BST:
//!
//!   B
//!  / \
//! A   D
//!
//! Would be written as:
//!
//! Node(
//!   "B",
//!   Box::new(Node("A", 
//!     Box::new(Leaf), Box::new(Leaf))),
//!   Box::new(Node("D", 
//!     Box::new(Leaf), Box::new(Leaf))));

use std::fmt::{self, Debug, Display};
use std::mem;

#[derive(PartialEq, Eq, Clone)]
pub enum BST<T> {
    /// A leaf is the bottom of the tree, it contains no data
    Leaf,
    /// A node contains a datum of type T, then left and right children.
    /// Those children are in boxes, so the BST type has a fixed size.
    Node(T, Box<BST<T>>, Box<BST<T>>),
}

impl<T: PartialOrd + Display> BST<T> {
    /// P1a: `len` computes the number of nodes in the BST `self`.
    ///
    /// For this and all other methods, you can test it by running `cargo test <method name>`.
    pub fn len(&self) -> i32 {
        unimplemented!()
    }

    /// P1b: `insert` takes a value of type T, and inserts it into the BST.
    /// `insert` must maintain the sorted invariant of the BST.
    ///
    /// This method should *NOT* be fancy, i.e. involve rotating or rebalancing
    /// the tree. The reference solution is 7 lines long.
    pub fn insert(&mut self, t: T) {
        unimplemented!()
    }

    /// P1c: `search` takes a query of type &T, and returns the smallest element
    /// greater than or equal to the query element. If no such element exists, then return None.
    pub fn search(&self, query: &T) -> Option<&T> {
        unimplemented!()
    }

    /// P1d [CHALLENGE PROBLEM, try if you're feeling up to it!]
    ///
    /// `rebalance` performs a single rebalancing operation on the BST in-place (if applicable).
    /// The rebalancing algorithm is to lift the closest element on the larger sub-tree up to the root,
    /// rotating the former root as the root of a subtree. Here are three examples:
    ///
    /// ```text
    ///
    ///      D               C
    ///     / \             / \
    ///    B   E   -->     B   D
    ///   / \             /     \
    ///  A   C           A       E
    ///
    /// A                  B
    ///  \                / \
    ///   B     -->      A   C
    ///    \                  \
    ///     C                  D
    ///      \
    ///       D
    ///       
    ///      E               D
    ///     / \             / \
    ///    B   F   -->     B   E
    ///     \               \   \
    ///      D               C   F
    ///     /
    ///    C
    ///
    /// ```
    ///
    /// Our criterion for rebalancing is if the size of one subtree is at least two greater than the other subtree.
    /// In the last example, the subtrees of E are size 3 and 1, respectively, so we rebalance from left to right.
    /// Here, the rebalancing procedure walks down the right spine of the left subtree to find the element D.
    /// Then it moves that element up to the root, and replaces D with its left subtree of C.
    /// The second example shows the same procedure, but rebalancing right to left.
    ///
    /// This task is difficult to implement in Rust. You cannot clone any data within the tree,
    /// and you must perform surgery on the tree without ever having "temporarily" NULL pointers. You will most likely
    /// want to use `mem::replace` to implement this function: https://doc.rust-lang.org/std/mem/fn.replace.html
    ///
    /// You will also want to implement helper functions to perform the search for the new BST root.
    pub fn rebalance(&mut self) {
        unimplemented!()
    }


    /// Provided helper function that gives a nice visual representation of a BST.
    /// You can print any BST by doing `println!("{tree:?}")`.
    ///
    /// Adapted from https://docs.rs/ascii_tree
    fn fmt_levels(&self, f: &mut fmt::Formatter<'_>, level: Vec<usize>) -> fmt::Result {
        use BST::*;
        const EMPTY: &str = "   ";
        const EDGE: &str = " └─";
        const PIPE: &str = " │ ";
        const BRANCH: &str = " ├─";

        let maxpos = level.len();
        let mut second_line = String::new();
        for (pos, l) in level.iter().enumerate() {
            let last_row = pos == maxpos - 1;
            if *l == 1 {
                if !last_row {
                    write!(f, "{}", EMPTY)?
                } else {
                    write!(f, "{}", EDGE)?
                }
                second_line.push_str(EMPTY);
            } else {
                if !last_row {
                    write!(f, "{}", PIPE)?
                } else {
                    write!(f, "{}", BRANCH)?
                }
                second_line.push_str(PIPE);
            }
        }

        match self {
            Node(s, l, r) => {
                let mut d = 2;
                writeln!(f, " {s}")?;
                for t in &[l, r] {
                    let mut lnext = level.clone();
                    lnext.push(d);
                    d -= 1;
                    t.fmt_levels(f, lnext)?;
                }
            }
            Leaf => writeln!(f)?,
        }
        Ok(())
    }
}

impl<T: Debug + Display + PartialOrd> fmt::Debug for BST<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_levels(f, vec![])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use BST::*;

    lazy_static! {
        static ref TEST_TREE: BST<&'static str> = {
            Node(
                "B",
                Box::new(Node("A", Box::new(Leaf), Box::new(Leaf))),
                Box::new(Node("C", Box::new(Leaf), Box::new(Leaf))),
            )
        };
    }

    #[test]
    fn len_test() {
        let heap_tree: Box<BST<i32>> = Box::new(BST::Leaf);
        match &*heap_tree {
            BST::Leaf => {}
            BST::Node(t, _, _) => {
                println!("{}", t);
            }
        };

        assert_eq!(TEST_TREE.len(), 3);
    }

    #[test]
    fn insertion_test() {
        let mut t = TEST_TREE.clone();
        t.insert("E");
        assert_eq!(
            t,
            Node(
                "B",
                Box::new(Node("A", Box::new(Leaf), Box::new(Leaf))),
                Box::new(Node(
                    "C",
                    Box::new(Leaf),
                    Box::new(Node("E", Box::new(Leaf), Box::new(Leaf)))
                )),
            )
        );
    }

    #[test]
    fn search_test() {
        let mut t = TEST_TREE.clone();
        t.insert("E");
        assert!(t.search(&"D") == Some(&"E"));
        assert!(t.search(&"C") == Some(&"C"));
        assert!(t.search(&"F") == None);
    }

    #[test]
    fn rebalance1_test() {
        let mut t = Node(
            "D",
            Box::new(Node(
                "B",
                Box::new(Node("A", Box::new(Leaf), Box::new(Leaf))),
                Box::new(Node("C", Box::new(Leaf), Box::new(Leaf))),
            )),
            Box::new(Node("E", Box::new(Leaf), Box::new(Leaf))),
        );

        let t2 = Node(
            "C",
            Box::new(Node(
                "B",
                Box::new(Node("A", Box::new(Leaf), Box::new(Leaf))),
                Box::new(Leaf),
            )),
            Box::new(Node(
                "D",
                Box::new(Leaf),
                Box::new(Node("E", Box::new(Leaf), Box::new(Leaf))),
            )),
        );

        t.rebalance();
        assert_eq!(t, t2);
    }

    #[test]
    fn rebalance2_test() {
        let mut t = Node(
            "A",
            Box::new(Leaf),
            Box::new(Node(
                "B",
                Box::new(Leaf),
                Box::new(Node(
                    "C",
                    Box::new(Leaf),
                    Box::new(Node("D", Box::new(Leaf), Box::new(Leaf))),
                )),
            )),
        );

        let t2 = Node(
            "B",
            Box::new(Node("A", Box::new(Leaf), Box::new(Leaf))),
            Box::new(Node(
                "C",
                Box::new(Leaf),
                Box::new(Node("D", Box::new(Leaf), Box::new(Leaf))),
            )),
        );

        t.rebalance();
        assert_eq!(t, t2);
    }

    #[test]
    fn rebalance3_test() {
        let mut t = Node(
            "E",
            Box::new(Node(
                "B",
                Box::new(Leaf),
                Box::new(Node(
                    "D",
                    Box::new(Node("C", Box::new(Leaf), Box::new(Leaf))),
                    Box::new(Leaf),
                )),
            )),
            Box::new(Node("F", Box::new(Leaf), Box::new(Leaf))),
        );

        let t2 = Node(
            "D",
            Box::new(Node(
                "B",
                Box::new(Leaf),
                Box::new(Node("C", Box::new(Leaf), Box::new(Leaf))),
            )),
            Box::new(Node(
                "E",
                Box::new(Leaf),
                Box::new(Node("F", Box::new(Leaf), Box::new(Leaf))),
            )),
        );

        t.rebalance();
        assert_eq!(t, t2);
    }
}
