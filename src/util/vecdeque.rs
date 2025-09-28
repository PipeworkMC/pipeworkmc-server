use std::collections::VecDeque;


pub(crate) trait VecDequeExt {
    type Inner;

    fn pop_many_front(&mut self, count : usize);
}

impl<T> VecDequeExt for VecDeque<T> {
    type Inner = T;

    fn pop_many_front(&mut self, count : usize) {
        for _ in 0..count { self.pop_front(); }
    }
}
