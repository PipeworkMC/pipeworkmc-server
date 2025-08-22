use std::collections::VecDeque;


pub trait OptionExt {
    type Inner;

    fn get_or_maybe_insert_with<F>(&mut self, f : F) -> Option<&mut Self::Inner>
    where
        F : FnOnce() -> Option<Self::Inner>;

}

impl<T> OptionExt for Option<T> {
    type Inner = T;

    fn get_or_maybe_insert_with<F>(&mut self, f : F) -> Option<&mut Self::Inner>
    where
        F : FnOnce() -> Option<Self::Inner>
    { match (self) {
        Some(inner) => Some(inner),
        None        => { match (f()) {
            Some(new_inner) => Some(self.insert(new_inner)),
            None            => None
        } },
    } }

}


pub trait VecDequeExt {
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
