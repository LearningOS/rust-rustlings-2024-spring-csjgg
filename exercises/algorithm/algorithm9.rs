use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
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
        self.items.push(value);
        self.count += 1;
        self.percolate_up();
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
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        if right_idx <= self.count && (self.comparator)(&self.items[right_idx], &self.items[left_idx]) {
            right_idx
        } else {
            left_idx
        }
    }

    fn percolate_up(&mut self) {
        let mut idx = self.count;
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]) {
            self.swap_items(idx, self.parent_idx(idx));
            idx = self.parent_idx(idx);
        }
    }
    
    fn swap_items(&mut self, idx1: usize, idx2: usize) {
        self.items.swap(idx1, idx2);
    }

    fn percolate_down(&mut self, idx: usize) {
        let mut current_idx = idx;
        while self.children_present(current_idx) {
            let smallest_child = self.smallest_child_idx(current_idx);
            if (self.comparator)(&self.items[smallest_child], &self.items[current_idx]) {
                self.items.swap(smallest_child, current_idx);
                current_idx = smallest_child;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.items.swap(1, self.count);
        let min_item = self.items.pop().unwrap();
        self.count -= 1;
        if self.count > 1 {
            self.percolate_down(1);
        }
        Some(min_item)
    }
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
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