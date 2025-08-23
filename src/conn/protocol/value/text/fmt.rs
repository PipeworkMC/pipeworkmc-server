use crate::conn::protocol::value::text::{
    Text,
    TextComponent,
    TextContent
};
use core::fmt::{ self,
    Display,
    Formatter
};


// TODO: No colour env/setting.


const ESC : &'static str = "\x1b";

impl Display for Text {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        if (self.components.is_empty()) { return Ok(()); }
        write!(f, "{ESC}[0m")?;
        for component in &*self.components {
            component.fmt_unwrapped(f)?;
            write!(f, "{ESC}[0m")?;
        }
        Ok(())
    }
}


impl Display for TextComponent {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{ESC}[0m")?;
        self.fmt_unwrapped(f)?;
        write!(f, "{ESC}[0m")?;
        Ok(())
    }
}
impl TextComponent {
    fn fmt_unwrapped(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{ESC}[38;2;{};{};{}m", self.colour.r, self.colour.g, self.colour.b)?;
        if (self.bold) { write!(f, "{ESC}[1m")?; }
        if (self.italic) { write!(f, "{ESC}[3m")?; }
        if (self.underline) { write!(f, "{ESC}[4m")?; }
        if (self.strike) { write!(f, "{ESC}[9m")?; }
        if (self.obfuscate) { write!(f, "{ESC}[5m")?; }
        if let Some(colour) = &self.shadow {
            write!(f, "{ESC}[48;2;{};{};{}m", colour.r, colour.g, colour.b)?;
        }
        write!(f, "{}", self.content)?;
        Ok(())
    }
}


impl Display for TextContent {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        match (self) {
            TextContent::Literal    { text    } => write!(f, "{}", text),
            TextContent::Translated { key, .. } => write!(f, "{}", key),
            TextContent::Keybind    { id      } => write!(f, "{}", id)
        }
    }
}
