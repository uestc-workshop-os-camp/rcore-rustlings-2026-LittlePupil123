/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default+ std::cmp::PartialOrd,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default+ std::cmp::PartialOrd,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count+=1;
        self.items.push(value);
        let mut tmp=self.count;
        loop{
            if (self.len()==1 || tmp==1) {break;}
            if (self.comparator)(&self.items[tmp],&self.items[self.parent_idx(tmp)]) {
                self.items.swap(tmp,tmp/2);
                tmp = tmp/2;
            }else{
                break;
            }
            
        }
        
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		if self.items[self.left_child_idx(idx)]>=self.items[self.right_child_idx(idx)] {
            self.right_child_idx(idx)
        }else {self.left_child_idx(idx)}
    }
}

impl<T> Heap<T> //初始化，分两种情况，建立大根堆还是小根堆
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+ std::cmp::PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<T> { //取堆顶，向下调整
        //TODO
        if self.is_empty() {
            return None;
        }

        self.items.swap(1,self.count);
		let res=self.items.pop();
        self.count-=1;
        
        //向下调整
        let mut tmp=1;
        loop{
            if !self.children_present(tmp) {break;}
            if self.right_child_idx(tmp)>=self.count {
                if (self.comparator)(&self.items[self.left_child_idx(tmp)],&self.items[tmp]){
                    self.items.swap(tmp,tmp*2);
                    tmp=self.left_child_idx(tmp);
                    continue;
                }else {break;} //当只有左子树时，若左子树比父节点小，则交换，否则结束
            }
            if (self.comparator)(&self.items[self.right_child_idx(tmp)],&self.items[self.left_child_idx(tmp)]){
                //与右子树比较
                if (self.comparator)(&self.items[self.right_child_idx(tmp)],&self.items[tmp]){
                    self.items.swap(tmp,tmp*2+1);
                    tmp=self.right_child_idx(tmp);
                }else {break;}
            }else{
                //与左子树比较
                if (self.comparator)(&self.items[self.left_child_idx(tmp)],&self.items[tmp]){
                    self.items.swap(tmp,tmp*2);
                    tmp=self.left_child_idx(tmp);
                }else {break;}
            }
        }
        res
    }
}

pub struct MinHeap;

impl MinHeap {//小根堆
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {//大根堆
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}