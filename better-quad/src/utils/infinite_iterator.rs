/// Infinite repeating iterator around a Vec<T>. Immutable.
///
/// # Panic
///
/// Panics if you try to create an empty infinite iterator.
pub struct InfiniteIterator<T> {
    items: Vec<T>,
    // Invariant: `current_index` is always a valid index into `items`.
    current_index: usize,
}

impl<T> InfiniteIterator<T> {
    fn from_vec(items: Vec<T>) -> Self {
        assert!(
            !items.is_empty(),
            "Can't use InfiniteIterator with empty vec"
        );
        Self {
            items,
            current_index: 0,
        }
    }

    pub fn current(&self) -> &T {
        &self.items[self.current_index]
    }

    pub fn current_mut(&mut self) -> &mut T {
        &mut self.items[self.current_index]
    }

    pub fn advance(&mut self) {
        self.current_index = (self.current_index + 1) % self.items.len();
    }

    pub fn raw(&self) -> (&Vec<T>, usize) {
        (&self.items, self.current_index)
    }
}

impl<T> From<Vec<T>> for InfiniteIterator<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::from_vec(vec)
    }
}

impl<T: Clone> From<&[T]> for InfiniteIterator<T> {
    fn from(value: &[T]) -> Self {
        Self::from_vec(value.to_vec())
    }
}
