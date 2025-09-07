use serde::ser::{
    Serialize as Ser,
    Serializer as Serer
};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Rgb {
    pub r : u8,
    pub g : u8,
    pub b : u8
}

impl Rgb {

    #[inline(always)]
    pub fn new(r : u8, g : u8, b : u8) -> Self {
        Self { r, g, b }
    }

    #[inline(always)]
    pub fn splat(v : u8) -> Self {
        Self { r : v, g : v, b : v }
    }

    pub const BLACK      : Self = Self { r :   0, g :   0, b :   0 };
    pub const DARK_BLUE  : Self = Self { r :   0, g :   0, b : 170 };
    pub const DARK_GREEN : Self = Self { r :   0, g : 170, b :   0 };
    pub const DARK_CYAN  : Self = Self { r :   0, g : 170, b : 170 };
    pub const DARK_RED   : Self = Self { r : 170, g :   0, b :   0 };
    pub const PURPLE     : Self = Self { r : 170, g :   0, b : 170 };
    pub const ORANGE     : Self = Self { r : 255, g : 170, b :   0 };
    pub const GREY       : Self = Self { r : 170, g : 170, b : 170 };
    pub const DARK_GREY  : Self = Self { r :  85, g :  85, b :  85 };
    pub const BLUE       : Self = Self { r :  85, g :  85, b : 255 };
    pub const GREEN      : Self = Self { r :  85, g : 255, b :  85 };
    pub const CYAN       : Self = Self { r :  85, g : 255, b : 255 };
    pub const RED        : Self = Self { r : 255, g :  85, b :  85 };
    pub const PINK       : Self = Self { r : 255, g :  85, b : 255 };
    pub const YELLOW     : Self = Self { r : 255, g : 255, b :  85 };
    pub const WHITE      : Self = Self { r : 255, g : 255, b : 255 };

}

impl Rgb {

    #[inline(always)]
    pub fn with_alpha(self, a : u8) -> Argb {
        Argb { a, r : self.r, g : self.g, b : self.b }
    }

    #[inline(always)]
    pub fn opaque(self) -> Argb {
        Argb { a : 255, r : self.r, g : self.g, b : self.b }
    }

    #[inline]
    pub fn to_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

}

impl Ser for Rgb {
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { self.to_u32().serialize(serer) }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Argb {
    pub a : u8,
    pub r : u8,
    pub g : u8,
    pub b : u8
}

impl Argb {

    #[inline(always)]
    pub fn new(a : u8, r : u8, g : u8, b : u8) -> Self {
        Self { a, r, g, b }
    }

    #[inline(always)]
    pub fn splat(a : u8, v : u8) -> Self {
        Self { a, r : v, g : v, b : v }
    }

    pub const BLACK       : Self = Self { a : 255, r :   0, g :   0, b :   0 };
    pub const DARK_BLUE   : Self = Self { a : 255, r :   0, g :   0, b : 170 };
    pub const DARK_GREEN  : Self = Self { a : 255, r :   0, g : 170, b :   0 };
    pub const DARK_CYAN   : Self = Self { a : 255, r :   0, g : 170, b : 170 };
    pub const DARK_RED    : Self = Self { a : 255, r : 170, g :   0, b :   0 };
    pub const PURPLE      : Self = Self { a : 255, r : 170, g :   0, b : 170 };
    pub const ORANGE      : Self = Self { a : 255, r : 255, g : 170, b :   0 };
    pub const GREY        : Self = Self { a : 255, r : 170, g : 170, b : 170 };
    pub const DARK_GREY   : Self = Self { a : 255, r :  85, g :  85, b :  85 };
    pub const BLUE        : Self = Self { a : 255, r :  85, g :  85, b : 255 };
    pub const GREEN       : Self = Self { a : 255, r :  85, g : 255, b :  85 };
    pub const CYAN        : Self = Self { a : 255, r :  85, g : 255, b : 255 };
    pub const RED         : Self = Self { a : 255, r : 255, g :  85, b :  85 };
    pub const PINK        : Self = Self { a : 255, r : 255, g :  85, b : 255 };
    pub const YELLOW      : Self = Self { a : 255, r : 255, g : 255, b :  85 };
    pub const WHITE       : Self = Self { a : 255, r : 255, g : 255, b : 255 };
    pub const TRANSPARENT : Self = Self { a :   0, r :   0, g :   0, b :   0 };

}

impl Argb {

    #[inline(always)]
    pub fn without_alpha(self) -> Rgb {
        Rgb { r : self.r, g : self.g, b : self.b }
    }

}

impl From<Rgb> for Argb {
    #[inline(always)]
    fn from(value : Rgb) -> Self { value.opaque() }
}
