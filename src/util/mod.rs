pub mod ext;
pub mod par_eventwriter;
pub mod redacted;


#[inline(always)]
pub(crate) fn slice_is_empty<T>(slice : &[T]) -> bool {
    slice.is_empty()
}
