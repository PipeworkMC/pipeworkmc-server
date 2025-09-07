use core::{
    hint::unreachable_unchecked,
    ops::Deref
};
use serde::{
    Serialize as Ser,
    Serializer as Serer
};


#[derive(Debug, Clone)]
pub enum BoxCow<'l, B>
where
    B : Clone + ?Sized + 'l
{
    Borrowed(&'l B),
    Owned(Box<B>)
}


impl<'l, B> BoxCow<'l, B>
where
    B : Clone + ?Sized + 'l
{

    pub fn to_mut(&mut self) -> &mut B {
        match (self) {
            Self::Borrowed(b)  => {
                *self = Self::Owned(Box::new(b.clone()));
                let Self::Owned(o) = self
                    else { unsafe { unreachable_unchecked() } };
                o
            },
            Self::Owned(o) => o,
        }
    }

    #[inline]
    pub fn into_owned(self) -> Box<B> {
        match (self) {
            Self::Borrowed(b) => Box::new(b.clone()),
            Self::Owned(o)    => o
        }
    }

}


impl<'l, B> Deref for BoxCow<'l, B>
where
    B : Clone + ?Sized + 'l
{
    type Target = B;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match (self) {
            Self::Borrowed(b) => b,
            Self::Owned(o)    => o
        }
    }
}


impl<'l, B> Ser for BoxCow<'l, B>
where
            B     : Clone + ?Sized + 'l,
    for<'k> &'k B : Ser
{
    #[inline]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { self.deref().serialize(serer) }
}
