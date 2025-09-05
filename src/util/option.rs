pub(crate) trait OptionExt {
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
