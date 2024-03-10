use std::collections::VecDeque;

pub struct SizedVector<T> {
    buffer: VecDeque<T>,
    max_size: usize,
}

impl<T> SizedVector<T> {
    pub fn new(max_size: usize) -> Self {
        SizedVector {
            buffer: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() >= self.max_size {
            // If buffer is full, remove the oldest element
            let _ = self.buffer.pop_front();
        }
        self.buffer.push_back(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.buffer.pop_front()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

pub struct SizedVectorIterator<'a, T> {
    inner: std::collections::vec_deque::Iter<'a, T>,
}

impl<'a, T> Iterator for SizedVectorIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T> IntoIterator for &'a SizedVector<T> {
    type Item = &'a T;
    type IntoIter = SizedVectorIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SizedVectorIterator {
            inner: self.buffer.iter(),
        }
    }
}

pub struct IntoSizedVectorIterator<T> {
    inner: std::collections::vec_deque::IntoIter<T>,
}

impl<T> Iterator for IntoSizedVectorIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T> IntoIterator for SizedVector<T> {
    type Item = T;
    type IntoIter = IntoSizedVectorIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoSizedVectorIterator {
            inner: self.buffer.into_iter(),
        }
    }
}
