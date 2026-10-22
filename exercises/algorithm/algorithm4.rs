/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord+Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        let mut Current = & mut self.root;
        match Current {
            Some(parent) =>{
                parent.insert(value);
            },
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
        

    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut Current = &self.root;
        loop{
            match Current{
                None => return false,
                Some(parent) =>{
                    if parent.value == value {return true}
                    
                    if parent.value > value {Current=&parent.left}
                    else if parent.value < value {Current=&parent.right}
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        let mut Current = self;
        loop{
            if value == Current.value {
                break;
            }
            if value < Current.value {
                // 1. 如果左边是空的，直接挂上去并 break
                if Current.left.is_none() {
                    Current.left = Some(Box::new(TreeNode::new(value)));
                    break;
                }
                // 2. 如果不空，游标往下走
                Current = Current.left.as_mut().unwrap();
            } else {
                if Current.right.is_none() {
                    Current.right = Some(Box::new(TreeNode::new(value)));
                    break;
                }
                Current = Current.right.as_mut().unwrap();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


