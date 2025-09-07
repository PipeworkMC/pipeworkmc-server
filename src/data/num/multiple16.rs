use core::fmt::{ self, Debug, Display, Formatter };
#[cfg(doc)]
use core::hint::unreachable_unchecked;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};


#[repr(transparent)]
pub struct Multiple16<T>(T)
where
    T : Multiple16ablePrimitive;

#[cfg(doc)]
impl<T> Multiple16<T>
where
    T : Multiple16ablePrimitive
{

    pub const fn new(n : T) -> Option<Self> { unsafe { unreachable_unchecked() } }

    pub const unsafe fn new_unchecked(n : T) -> Self { unsafe { unreachable_unchecked() } }

    pub const fn get(self) -> T { unsafe { unreachable_unchecked() } }

}

impl<T> Debug for Multiple16<T>
where
    T : Multiple16ablePrimitive
{
    #[inline(always)]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        <T as Debug>::fmt(&self.0, f)
    }
}

impl<T> Display for Multiple16<T>
where
    T : Multiple16ablePrimitive
{
    #[inline(always)]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        <T as Display>::fmt(&self.0, f)
    }
}

impl<T> Ser for Multiple16<T>
where
    T : Multiple16ablePrimitive
{
    #[inline(always)]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { <T as Ser>::serialize(&self.0, serer) }
}



#[allow(private_bounds)]
pub unsafe trait Multiple16ablePrimitive
where
    Self : Copy + Ser + Debug + Display + Sized + Sealed
{ }

trait Sealed { }

macro_rules! impl_multiple_16able_primitive_for {
    ($ty:ty, $ident:ident $(,)?) => {
        pub type $ident = Multiple16<$ty>;

        unsafe impl Multiple16ablePrimitive for $ty { }

        #[doc(hidden)]
        impl $ident {

            #[inline]
            pub const fn new(n : $ty) -> Option<Self> {
                if (n.rem_euclid(16) == 0) {
                    Some(Self(n))
                } else { None }
            }

            #[inline(always)]
            pub const unsafe fn new_unchecked(n : $ty) -> Self {
                Self(n)
            }

            #[inline(always)]
            pub const fn get(self) -> $ty { self.0 }

        }

        impl Sealed for $ty { }
    }
}
impl_multiple_16able_primitive_for!(u8, Multiple16U8);
impl_multiple_16able_primitive_for!(i8, Multiple16I8);
impl_multiple_16able_primitive_for!(u16, Multiple16U16);
impl_multiple_16able_primitive_for!(i16, Multiple16I16);
impl_multiple_16able_primitive_for!(u32, Multiple16U32);
impl_multiple_16able_primitive_for!(i32, Multiple16I32);
impl_multiple_16able_primitive_for!(u64, Multiple16U64);
impl_multiple_16able_primitive_for!(i64, Multiple16I64);
impl_multiple_16able_primitive_for!(u128, Multiple16U128);
impl_multiple_16able_primitive_for!(i128, Multiple16I128);
