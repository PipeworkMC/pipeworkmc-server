use crate::data::num::provider::IntProvider;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LightLevel(u8);

impl LightLevel {

    #[inline]
    pub const fn new(n : u8) -> Option<Self> {
        if (n < 16) {
            Some(Self(n))
        } else { None }
    }

    #[inline(always)]
    pub const unsafe fn new_unchecked(n : u8) -> Self {
        Self(n)
    }

    #[inline]
    pub const fn get(self) -> Self { Self(self.0) }

}

impl Ser for LightLevel {
    #[inline(always)]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { (self.0 as u32).serialize(serer) }
}


pub type LightLevelProvider<'l> = IntProvider<'l, LightLevel>;
