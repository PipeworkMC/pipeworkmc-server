mod option;
pub(crate) use option::*;

mod vecdeque;
pub(crate) use vecdeque::*;


#[inline(always)]
pub(crate) fn slice_is_empty<T>(slice : &[T]) -> bool {
    slice.is_empty()
}
