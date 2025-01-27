pub struct InfiniteIterator<T> {
    items: Vec<T>,
    // Soft invariant: `current_index` is always a valid index into `items`.
    // Invariant holds as long as items is non-empty.
    current_index: usize,
}

impl<T> InfiniteIterator<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current_index: 0,
        }
    }

    fn from_vec(items: Vec<T>) -> Self {
        Self {
            items,
            current_index: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn check_invariants(&self, method_name: &'static str) {
        if self.items.is_empty() {
            panic!("Can't call {method_name}() on empty InfiniteIterator");
        }
        if self.current_index >= self.items.len() {
            panic!("InfiniteIterator-Invariant-Bug: called {method_name}() with current_index={} and len={}.", self.current_index, self.items.len());
        }
    }

    pub fn current(&self) -> &T {
        self.check_invariants("current_mut");
        &self.items[self.current_index]
    }

    pub fn current_mut(&mut self) -> &mut T {
        self.check_invariants("current_mut");
        &mut self.items[self.current_index]
    }

    pub fn advance(&mut self) {
        self.check_invariants("advance");
        self.current_index = (self.current_index + 1) % self.items.len();
    }

    pub fn raw(&self) -> (&Vec<T>, usize) {
        self.check_invariants("raw");
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
