mod option;
pub(crate) use option::*;
mod vecdeque;
pub(crate) use vecdeque::*;


#[inline(always)]
pub(crate) fn slice_is_empty<T>(slice : &[T]) -> bool {
    slice.is_empty()
}

#[inline(always)]
pub(crate) fn is_default<T>(value : &T) -> bool
where
    T : Default + PartialEq
{ *value == T::default() }
