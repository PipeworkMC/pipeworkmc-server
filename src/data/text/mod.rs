use crate::data::{
    action::Action,
    colour::{ Rgb, Argb },
    ident::Ident
};
use core::{
    mem,
    ops::{ Add, AddAssign }
};
use std::borrow::Cow;


mod fmt;
mod ser;


#[derive(Clone, Debug)]
pub struct Text {
    pub components : Cow<'static, [TextComponent]>
}

#[derive(Clone, Debug)]
pub struct TextComponent {
    pub content   : TextContent,
    pub colour    : Rgb,
    pub font      : Option<Ident>,
    pub bold      : bool,
    pub italic    : bool,
    pub underline : bool,
    pub strike    : bool,
    pub obfuscate : bool,
    pub shadow    : Option<Argb>,
    pub insertion : Option<Cow<'static, str>>,
    pub on_click  : Option<Action>,
    pub tooltip   : Option<Text>
}

#[derive(Clone, Debug)]
pub enum TextContent {
    Literal {
        text : Cow<'static, str>
    },
    Translate {
        key      : Cow<'static, str>,
        fallback : Option<Cow<'static, str>>,
        with     : Cow<'static, [Text]>
    },
    Keybind {
        id : Cow<'static, str>
    }
}


impl Text {

    #[inline]
    pub fn literal<S>(text : S) -> Self
    where
        S : Into<Cow<'static, str>>
    { TextContent::Literal {
        text : text.into()
    }.into() }

    #[inline]
    pub fn translate<S>(key : S) -> Self
    where
        S : Into<Cow<'static, str>>
    { TextContent::Translate {
        key      : key.into(),
        fallback : None,
        with     : Cow::Borrowed(&[])
    }.into() }

    #[inline]
    pub fn translate_or<S, F>(key : S, fallback : F) -> Self
    where
        S : Into<Cow<'static, str>>,
        F : Into<Cow<'static, str>>
    { TextContent::Translate {
        key      : key.into(),
        fallback : Some(fallback.into()),
        with     : Cow::Borrowed(&[])
    }.into() }

    #[inline]
    pub fn translate_with<S, W>(key : S, with : W) -> Self
    where
        S : Into<Cow<'static, str>>,
        W : Into<Cow<'static, [Text]>>
    { TextContent::Translate {
        key      : key.into(),
        fallback : None,
        with     : with.into()
    }.into() }

    #[inline]
    pub fn translate_with_or<S, W, F>(key : S, with : W, fallback : F) -> Self
    where
        S : Into<Cow<'static, str>>,
        W : Into<Cow<'static, [Text]>>,
        F : Into<Cow<'static, str>>
    { TextContent::Translate {
        key      : key.into(),
        fallback : Some(fallback.into()),
        with     : with.into()
    }.into() }

    #[inline]
    pub fn keybind<S>(id : S) -> Self
    where
        S : Into<Cow<'static, str>>
    { TextContent::Keybind {
        id : id.into()
    }.into() }

}

impl Text {

    pub fn apply<F>(mut self, mut f : F) -> Self
    where
        F : FnMut(&mut TextComponent)
    {
        let mut components = self.components.into_owned();
        for component in &mut components {
            f(component);
        }
        self.components = Cow::Owned(components);
        self
    }

}


impl TextComponent {

    pub const EMPTY : Self = Self {
        content   : TextContent::Literal { text : Cow::Borrowed("") },
        colour    : Rgb::WHITE,
        font      : None,
        bold      : false,
        italic    : false,
        underline : false,
        strike    : false,
        obfuscate : false,
        shadow    : None,
        insertion : None,
        on_click  : None,
        tooltip   : None
    };

}

impl Default for TextComponent {
    #[inline(always)]
    fn default() -> Self { Self::EMPTY }
}

impl From<&'static str> for TextComponent {
    fn from(value : &'static str) -> Self { Self {
        content : TextContent::Literal { text : Cow::Borrowed(value) },
        ..Self::EMPTY
    } }
}
impl From<String> for TextComponent {
    fn from(value : String) -> Self { Self {
        content : TextContent::Literal { text : Cow::Owned(value) },
        ..Self::EMPTY
    } }
}
impl From<Cow<'static, str>> for TextComponent {
    fn from(value : Cow<'static, str>) -> Self { Self {
        content : TextContent::Literal { text : value },
        ..Self::EMPTY
    } }
}
impl From<TextContent> for TextComponent {
    fn from(value : TextContent) -> Self { Self {
        content : value,
        ..Self::EMPTY
    } }
}

impl<T> From<T> for Text
where
    TextComponent : From<T>
{ fn from(value : T) -> Self {
    Self { components : Cow::Owned(vec![value.into()]) }
} }


pub trait TextFormatted
where
    Self : Sized
{
    fn colour<C>(self, colour : C) -> Text
        where C : Into<Rgb>;
    fn black(self) -> Text { self.colour(Rgb::BLACK) }
    fn dark_blue(self) -> Text { self.colour(Rgb::DARK_BLUE) }
    fn dark_green(self) -> Text { self.colour(Rgb::DARK_GREEN) }
    fn dark_cyan(self) -> Text { self.colour(Rgb::DARK_CYAN) }
    fn dark_red(self) -> Text { self.colour(Rgb::DARK_RED) }
    fn purple(self) -> Text { self.colour(Rgb::PURPLE) }
    fn orange(self) -> Text { self.colour(Rgb::ORANGE) }
    fn grey(self) -> Text { self.colour(Rgb::GREY) }
    fn dark_grey(self) -> Text { self.colour(Rgb::DARK_GREY) }
    fn blue(self) -> Text { self.colour(Rgb::BLUE) }
    fn green(self) -> Text { self.colour(Rgb::GREEN) }
    fn cyan(self) -> Text { self.colour(Rgb::CYAN) }
    fn red(self) -> Text { self.colour(Rgb::RED) }
    fn pink(self) -> Text { self.colour(Rgb::PINK) }
    fn yellow(self) -> Text { self.colour(Rgb::YELLOW) }
    fn white(self) -> Text { self.colour(Rgb::WHITE) }
    fn font<R>(self, resource : R) -> Text
        where R : Into<Ident>;
    fn no_font(self) -> Text;
    fn bold(self) -> Text;
    fn no_bold(self) -> Text;
    fn italic(self) -> Text;
    fn no_italic(self) -> Text;
    fn underline(self) -> Text;
    fn no_underline(self) -> Text;
    fn strike(self) -> Text;
    fn no_strike(self) -> Text;
    fn obfuscate(self) -> Text;
    fn no_obfuscate(self) -> Text;
    fn shadow<C>(self, colour : C) -> Text
        where C : Into<Argb>;
    fn no_shadow(self) -> Text { self.shadow(Argb::TRANSPARENT) }
    fn default_shadow(self) -> Text;
    fn insertion<S>(self, text : S) -> Text
        where S : Into<Cow<'static, str>>;
    fn no_insertion(self) -> Text;
    fn on_click(self, action : Action) -> Text;
    fn no_on_click(self) -> Text;
    fn tooltip<S>(self, text : S) -> Text
        where S : Into<Text>;
    fn no_tooltip(self) -> Text;
    fn reset(self) -> Text;
}

impl TextFormatted for Text {
    fn colour<C>(self, colour : C) -> Text
    where C : Into<Rgb> {
        let colour = colour.into();
        self.apply(|component| { component.colour = colour; })
    }
    fn font<R>(self, resource : R) -> Text
    where R : Into<Ident> {
        let resource = resource.into();
        self.apply(|component| { component.font = Some(resource.clone()); })
    }
    fn no_font(self) -> Text {
        self.apply(|component| { component.font = None; })
    }
    fn bold(self) -> Text {
        self.apply(|component| { component.bold = true; })
    }
    fn no_bold(self) -> Text {
        self.apply(|component| { component.bold = false; })
    }
    fn italic(self) -> Text {
        self.apply(|component| { component.italic = true; })
    }
    fn no_italic(self) -> Text {
        self.apply(|component| { component.italic = true; })
    }
    fn underline(self) -> Text {
        self.apply(|component| { component.underline = true; })
    }
    fn no_underline(self) -> Text {
        self.apply(|component| { component.underline = true; })
    }
    fn strike(self) -> Text {
        self.apply(|component| { component.strike = true; })
    }
    fn no_strike(self) -> Text {
        self.apply(|component| { component.strike = true; })
    }
    fn obfuscate(self) -> Text {
        self.apply(|component| { component.obfuscate = true; })
    }
    fn no_obfuscate(self) -> Text {
        self.apply(|component| { component.obfuscate = true; })
    }
    fn shadow<C>(self, colour : C) -> Text
    where C : Into<Argb> {
        let colour = colour.into();
        self.apply(|component| { component.shadow = Some(colour); })
    }
    fn no_shadow(self) -> Text {
        self.apply(|component| { component.shadow = Some(Argb::TRANSPARENT); })
    }
    fn default_shadow(self) -> Text {
        self.apply(|component| { component.shadow = None; })
    }
    fn insertion<S>(self, text : S) -> Text
    where S : Into<Cow<'static, str>> {
        let text = text.into();
        self.apply(|component| { component.insertion = Some(text.clone()); })
    }
    fn no_insertion(self) -> Text {
        self.apply(|component| { component.insertion = None; })
    }
    fn on_click(self, action : Action) -> Text {
        self.apply(|component| { component.on_click = Some(action.clone()); })
    }
    fn no_on_click(self) -> Text {
        self.apply(|component| { component.on_click = None; })
    }
    fn tooltip<S>(self, text : S) -> Text
    where S : Into<Text> {
        let text = text.into();
        self.apply(|component| { component.tooltip = Some(text.clone()); })
    }
    fn no_tooltip(self) -> Text {
        self.apply(|component| { component.tooltip = None; })
    }
    fn reset(self) -> Text {
        self.apply(|component| {
            component.colour    = Rgb::WHITE;
            component.font      = None;
            component.bold      = false;
            component.italic    = false;
            component.underline = false;
            component.strike    = false;
            component.obfuscate = false;
            component.shadow    = None;
            component.insertion = None;
            component.on_click  = None;
            component.tooltip   = None;
        })
    }
}

impl<T> TextFormatted for T
where
    TextComponent : From<T>
{
    #[inline(always)]
    fn colour<C>(self, colour : C) -> Text
    where C : Into<Rgb> { Text::from(self).colour(colour) }
    #[inline(always)]
    fn font<R>(self, resource : R) -> Text
    where R : Into<Ident> { Text::from(self).font(resource) }
    #[inline(always)]
    fn no_font(self) -> Text { Text::from(self).no_font() }
    #[inline(always)]
    fn bold(self) -> Text { Text::from(self).bold() }
    #[inline(always)]
    fn no_bold(self) -> Text { Text::from(self).no_bold() }
    #[inline(always)]
    fn italic(self) -> Text { Text::from(self).italic() }
    #[inline(always)]
    fn no_italic(self) -> Text { Text::from(self).no_italic() }
    #[inline(always)]
    fn underline(self) -> Text { Text::from(self).underline() }
    #[inline(always)]
    fn no_underline(self) -> Text { Text::from(self).no_underline() }
    #[inline(always)]
    fn strike(self) -> Text { Text::from(self).strike() }
    #[inline(always)]
    fn no_strike(self) -> Text { Text::from(self).no_strike() }
    #[inline(always)]
    fn obfuscate(self) -> Text { Text::from(self).obfuscate() }
    #[inline(always)]
    fn no_obfuscate(self) -> Text { Text::from(self).no_obfuscate() }
    #[inline(always)]
    fn shadow<C>(self, colour : C) -> Text
    where C : Into<Argb> { Text::from(self).shadow(colour) }
    #[inline(always)]
    fn no_shadow(self) -> Text { Text::from(self).no_shadow() }
    #[inline(always)]
    fn default_shadow(self) -> Text { Text::from(self).default_shadow() }
    #[inline(always)]
    fn insertion<S>(self, text : S) -> Text
    where S : Into<Cow<'static, str>> { Text::from(self).insertion(text) }
    #[inline(always)]
    fn no_insertion(self) -> Text { Text::from(self).no_insertion() }
    #[inline(always)]
    fn on_click(self, action : Action) -> Text { Text::from(self).on_click(action) }
    #[inline(always)]
    fn no_on_click(self) -> Text { Text::from(self).no_on_click() }
    #[inline(always)]
    fn tooltip<S>(self, text : S) -> Text
    where S : Into<Text> { Text::from(self).tooltip(text) }
    #[inline(always)]
    fn no_tooltip(self) -> Text { Text::from(self).no_tooltip() }
    #[inline(always)]
    fn reset(self) -> Text { Text::from(self).reset() }
}


impl<T> Add<T> for Text
where
    T : Into<Text>
{
    type Output = Text;
    #[inline(always)]
    fn add(mut self, rhs : T) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> AddAssign<T> for Text
where
    T : Into<Text>
{
    fn add_assign(&mut self, rhs : T) {
        let mut components = mem::replace(&mut self.components, Cow::Borrowed(&[])).into_owned();
        components.extend_from_slice(&rhs.into().components);
        self.components = Cow::Owned(components);
    }
}

macro impl_addtext_for($ty:ty) {
    impl Add<Text> for $ty {
        type Output = Text;
        fn add(self, mut rhs : Text) -> Self::Output {
            let mut components = rhs.components.into_owned();
            components.insert(0, self.into());
            rhs.components = Cow::Owned(components);
            rhs
        }
    }
}
 impl_addtext_for!(&'static str);
 impl_addtext_for!(String);
 impl_addtext_for!(Cow<'static, str>);
 impl_addtext_for!(TextComponent);
 impl_addtext_for!(TextContent);
