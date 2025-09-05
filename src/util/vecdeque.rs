use std::collections::VecDeque;


pub(crate) trait VecDequeExt {
    type Inner;

    fn pop_many_front(&mut self, count : usize);

    unsafe fn pop_many_front_into_unchecked(&mut self, count : usize) -> Vec<Self::Inner>;
}

impl<T> VecDequeExt for VecDeque<T> {
    type Inner = T;

    fn pop_many_front(&mut self, count : usize) {
        for _ in 0..count { self.pop_front(); }
    }

    unsafe fn pop_many_front_into_unchecked(&mut self, count : usize) -> Vec<Self::Inner> {
        let mut b = Vec::with_capacity(count);
        for _ in 0..count {
            b.push(unsafe { self.pop_front().unwrap_unchecked() })
        }
        b
    }

}
