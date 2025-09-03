use crate::conn::protocol::codec::{
    decode::{
        PacketDecode,
        DecodeBuf,
        string::StringDecodeError
    },
    encode::{
        PacketEncode,
        EncodeBuf
    }
};
use core::fmt::{ self, Display, Debug, Formatter };
use std::borrow::Cow;
use serde::ser::{
    Serialize as Ser,
    Serializer as Serer
};


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    joined    : Cow<'static, str>,
    split_idx : usize
}

impl Display for Ident {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.joined)
    }
}

impl Debug for Ident {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.joined)
    }
}


impl Ident {

    #[inline]
    pub fn namespace(&self) -> &str {
        unsafe { self.joined.get_unchecked(0..self.split_idx) }
    }

    #[inline]
    pub fn path(&self) -> &str {
        unsafe { self.joined.get_unchecked((self.split_idx + 1)..) }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str { &self.joined }

}

impl Ident {

    pub unsafe fn new_unchecked<S>(joined : S) -> Self
    where
        S : Into<Cow<'static, str>>
    {
        let joined = joined.into();
        let Some(split_idx) = joined.as_bytes().iter().position(|&c| c == b':')
            else { panic!("Ident missing separator character"); };
        unsafe { Self::new_unchecked_manual(joined, split_idx) }
    }

    #[inline(always)]
    const unsafe fn new_unchecked_manual(joined : Cow<'static, str>, split_idx : usize) -> Self {
        Self { joined, split_idx }
    }

    #[track_caller]
    #[inline]
    pub const fn new(s : &'static str) -> Self {
        match (Self::new_checked(s)) {
            Ok(ident) => ident,
            Err(err)  => match (err) {
                IdentValidateError::NotAscii       => panic!("Ident contains non-ASCII characters"),
                IdentValidateError::EmptyComponent => panic!("Ident contains empty component"),
                IdentValidateError::BadChar(_)     => panic!("Ident component contains invalid character"),
                IdentValidateError::NoSeparator    => panic!("Ident missing separator character")
            }
        }
    }

    #[inline]
    pub const fn new_checked(s : &'static str) -> Result<Self, IdentValidateError> {
        match (Self::validate_joined(s)) {
            Ok(split_idx) => Ok(unsafe { Self::new_unchecked_manual(Cow::Borrowed(s), split_idx) }),
            Err(err)      => Err(err)
        }
    }

    #[inline(always)]
    pub fn new_from_pair(namespace : &str, path : &str) -> Result<Self, IdentValidateError> {
        Self::try_from((namespace, path,))
    }

    const fn validate_joined(joined : &str) -> Result<usize, IdentValidateError> {
        if (! joined.is_ascii()) {
            return Err(IdentValidateError::NotAscii);
        }
        let mut i = 0;
        while (i < joined.len()) {
            let ch = joined.as_bytes()[i];
            if (ch == b':') {
                if (i == 0) {
                    return Err(IdentValidateError::EmptyComponent);
                }
                return match (Self::validate_path(joined, i + 1)) {
                    Ok(()) => Ok(i),
                    Err(err) => Err(err),
                };
            } else if (! Self::is_valid_component_char(ch)) {
                return Err(IdentValidateError::BadChar(ch as char));
            }
            i += 1;
        }
        Err(IdentValidateError::NoSeparator)
    }

    #[inline]
    const fn validate_path(joined : &str, mut i : usize) -> Result<(), IdentValidateError> {
        let mut component_len = 0usize;
        while (i < joined.len()) {
            let ch = joined.as_bytes()[i];
            if (ch == b'/') {
                if (component_len == 0) {
                    return Err(IdentValidateError::EmptyComponent);
                }
                component_len = 0;
            } else if (Self::is_valid_component_char(ch)) {
                component_len += 1;
            } else {
                return Err(IdentValidateError::BadChar(ch as char));
            }
            i += 1;
        }
        if (component_len == 0) {
            return Err(IdentValidateError::EmptyComponent);
        }
        Ok(())
    }

    #[inline(always)]
    const fn is_valid_component_char(ch : u8) -> bool {
        (ch >= b'a' && ch <= b'z') || (ch >= b'0' && ch <= b'9') || ch == b'.' || ch == b'-' || ch == b'_'
    }

}


impl TryFrom<Cow<'static, str>> for Ident {
    type Error = IdentValidateError;
    #[inline]
    fn try_from(s : Cow<'static, str>) -> Result<Self, Self::Error> {
        let split_idx = Self::validate_joined(&s)?;
        Ok(unsafe { Self::new_unchecked_manual(s, split_idx) })
    }
}
impl TryFrom<&'static str> for Ident {
    type Error = IdentValidateError;
    #[inline(always)]
    fn try_from(s : &'static str) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Borrowed(s))
    }
}
impl TryFrom<String> for Ident {
    type Error = IdentValidateError;
    #[inline(always)]
    fn try_from(s : String) -> Result<Self, Self::Error> {
        Self::try_from(Cow::Owned(s))
    }
}
impl<N, P> TryFrom<(N, P,)> for Ident
where
    N : AsRef<str>,
    P : AsRef<str>
{
    type Error = IdentValidateError;
    #[inline]
    fn try_from((n, p,) : (N, P,)) -> Result<Self, Self::Error> {
        Self::try_from(format!("{}:{}", n.as_ref(), p.as_ref()))
    }
}


impl Ser for Ident {
    #[inline]
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { serer.serialize_str(self.as_str()) }
}


impl PacketDecode for Ident {
    type Error = IdentDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let s = <String>::decode(buf).map_err(IdentDecodeError::String)?;
        Self::try_from(s).map_err(IdentDecodeError::Validate)
    }
}

unsafe impl PacketEncode for Ident {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.joined.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.joined.encode(buf)
    } }

}


#[derive(Debug)]
pub enum IdentValidateError {
    NotAscii,
    EmptyComponent,
    BadChar(char),
    NoSeparator
}
impl Display for IdentValidateError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::NotAscii       => write!(f, "contains non-ASCII characters"),
        Self::EmptyComponent => write!(f, "contains empty component"),
        Self::BadChar(ch)    => write!(f, "component contains invalid character {ch:?}"),
        Self::NoSeparator    => write!(f, "missing separator character")
    } }
}

#[derive(Debug)]
pub enum IdentDecodeError {
    String(StringDecodeError),
    Validate(IdentValidateError)
}
impl Display for IdentDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::String(err)   => write!(f, "{err}"),
        Self::Validate(err) => write!(f, "{err}")
    } }
}
