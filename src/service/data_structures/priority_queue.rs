/// implementation guide from: https://rtoch.com/posts/priority-queue/
use std::fmt::{Debug, Formatter};
use std::ptr::eq;

/// Heap is a generic class that represents the heap data structure
pub struct Heap<T> {
    vector: Vec<T>,
}

impl<T: ToString> Debug for Heap<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let separator = " | ";
        let mut s = "Underlying Array -> | ".to_string();

        for element in &self.vector {
            s.push_str(element.to_string().as_str());
            if !eq(element, self.vector.last().unwrap()) {
                s.push_str(separator);
            }
        }
        write!(f, "{}", s)
    }
}

impl<T: Ord> Heap<T> {
    /// bubble_up moves a node up the heap if it is greater than its parents
    fn bubble_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let parent = Self::get_parent(index);
        if self.vector[parent] < self.vector[index] {
            self.vector.swap(parent, index);
            self.bubble_up(parent);
        }
    }

    /// bubble_down moves a node down the heap to its proper position
    fn bubble_down(&mut self, index: usize, boundary: usize) {
        let (left_child_idx, right_child_idx) = Self::get_children(index);

        let mut candidate = index;
        if left_child_idx < boundary && self.vector[left_child_idx] > self.vector[candidate] {
            candidate = left_child_idx
        }
        if right_child_idx < boundary && self.vector[right_child_idx] > self.vector[candidate] {
            candidate = right_child_idx
        }

        if candidate != index {
            self.vector.swap(candidate, index);
            self.bubble_down(candidate, boundary);
        }
    }

    /// new initializes a new heap
    pub fn new() -> Self {
        Self { vector: Vec::new() }
    }

    /// with_capacity initializes a new heap with a given capacity
    pub fn with_capacity(capacity: usize) -> Self
    where
        T: Ord,
    {
        Self {
            vector: Vec::with_capacity(capacity),
        }
    }

    /// size returns the number of elements in the heap
    pub fn size(&self) -> usize {
        self.vector.len()
    }

    /// is_empty returns true if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// push pushes a node in the heap
    pub fn push(&mut self, data: T) {
        self.vector.push(data);
        self.bubble_up(self.vector.len() - 1);
    }

    /// pop pops the root and fixes the tree
    pub fn pop(&mut self) -> Option<T> {
        match self.is_empty() {
            true => None,
            false => {
                let last_idx = self.vector.len() - 1;
                self.vector.swap(0, last_idx);
                self.bubble_down(0, last_idx);
                self.vector.pop()
            }
        }
    }

    pub fn into_sorted_vec(mut self) -> Vec<T> {
        for index in (0..self.size()).rev() {
            self.vector.swap(0, index);
            self.bubble_down(0, index)
        }
        self.vector
    }

    fn get_parent(index: usize) -> usize {
        (index - 1) / 2
    }

    fn get_children(index: usize) -> (usize, usize) {
        ((index * 2), (index * 2) + 1)
    }
}

#[cfg(test)]
mod test {
    use super::Heap;

    #[test]
    fn heap_works() {
        let mut heap: Heap<String> = Heap::new();
        heap.push("a".to_string());
        heap.push("b".to_string());
        heap.push("c".to_string());
    }

    #[test]
    fn push_pop_works() {
        let mut heap = Heap::new();
        heap.push(3);
        heap.push(1);
        heap.push(5);

        assert_eq!(heap.size(), 3);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(3));

        heap.push(4);
        heap.push(2);
        assert_eq!(heap.size(), 3);
        assert_eq!(heap.is_empty(), false);
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.is_empty(), true);
    }

    #[test]
    fn into_sorted_vec_works() {
        let mut heap = Heap::with_capacity(8);
        heap.push(5);
        heap.push(2);
        heap.push(7);
        heap.push(8);
        heap.push(1);
        heap.push(6);
        heap.push(4);
        heap.push(3);

        assert_eq!(heap.into_sorted_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
