use core::{
    error::Error as StdError,
    fmt::{ self, Display, Formatter }
};
use std::io::{ self, Write };
use serde::ser::{
    Error as SerError,
    Serialize as Ser
};


mod root;
use root::{ NbtRootSerer, TagWrite };
mod seq;
use seq::NbtSeqSerer;
mod map;
use map::NbtMapSerer;
mod str;
use str::NbtStrSerer;

mod never;

mod tag {
    pub const END      : u8 = 0;
    pub const BYTE     : u8 = 1;
    pub const SHORT    : u8 = 2;
    pub const INT      : u8 = 3;
    pub const LONG     : u8 = 4;
    pub const FLOAT    : u8 = 5;
    pub const DOUBLE   : u8 = 6;
    // pub const BARRAY   : u8 = 7;
    pub const STRING   : u8 = 8;
    pub const LIST     : u8 = 9;
    pub const COMPOUND : u8 = 10;
    // pub const IARRAY   : u8 = 11;
    // pub const LARRAY   : u8 = 12;
}


#[derive(Debug)]
pub enum NbtSerError {
    Io(io::Error),
    Custom(String)
}
impl From<io::Error> for NbtSerError {
    #[inline(always)]
    fn from(err : io::Error) -> Self {
        Self::Io(err)
    }
}

impl Display for NbtSerError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Io(err)     => write!(f, "{err}"),
        Self::Custom(err) => write!(f, "{err}")
    } }
}
impl StdError for NbtSerError { }
impl SerError for NbtSerError {
    fn custom<T>(msg : T) -> Self
    where
        T : Display
    {
        Self::Custom(format!("{msg}"))
    }
}


pub fn to_network<W, T>(mut writer : W, value : &T) -> Result<(), NbtSerError>
where
    W : Write,
    T : Ser
{
    value.serialize(NbtRootSerer::new(&mut writer, TagWrite::Tag))?;
    Ok(())
}
