use super::{
    Text,
    TextComponent,
    TextContent,
    TextStyle
};
use serde::ser::{
    Serialize as Ser,
    Serializer as Serer,
    SerializeSeq as _,
    SerializeMap as _
};


impl Ser for Text {
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    {
        let mut seq = serer.serialize_seq(Some(self.components.len()))?;
        for component in &*self.components {
            seq.serialize_element(component)?;
        }
        seq.end()
    }
}


impl Ser for TextComponent {
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    {
        let mut map = serer.serialize_map(None)?;
        map.serialize_entry("type", "text")?;
        map.serialize_entry("text", "")?;
        map.serialize_entry("extra", &[ExtraTextComponent(self)])?;
        map.end()
    }
}


struct ExtraTextComponent<'l>(&'l TextComponent);

impl Ser for ExtraTextComponent<'_> {
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    {
        let mut map = serer.serialize_map(None)?;

        match (&self.0.content) {
            TextContent::Literal { text } => {
                map.serialize_entry("type", "text")?;
                map.serialize_entry("text", text)?;
            },
            TextContent::Translate { key, fallback, with } => {
                map.serialize_entry("type", "translatable")?;
                map.serialize_entry("translate", key)?;
                if let Some(fallback) = fallback {
                    map.serialize_entry("fallback", fallback)?;
                }
                if (! with.is_empty()) {
                    map.serialize_entry("with", with)?;
                }
            },
            TextContent::Keybind { id } => {
                map.serialize_entry("type", "keybind")?;
                map.serialize_entry("keybind", id)?;
            }
        }

        self.0.style.serialize_map::<S>(&mut map)?;

        map.end()
    }
}


impl Ser for TextStyle {
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    {
        let mut map = serer.serialize_map(None)?;
        self.serialize_map::<S>(&mut map)?;
        map.end()
    }
}
impl TextStyle {
    fn serialize_map<S>(&self, map : &mut S::SerializeMap) -> Result<(), S::Error>
    where
        S : Serer
    {
        map.serialize_entry("color", &format!("#{:0>6x}",
            ((self.colour.r as u32) << 16) | ((self.colour.g as u32) << 8) | (self.colour.b as u32)
        ))?;

        if let Some(font) = &self.font {
            map.serialize_entry("font", font)?;
        }

        if (self.bold      ) { map.serialize_entry("bold"          , &true)?; }
        if (self.italic    ) { map.serialize_entry("italic"        , &true)?; }
        if (self.underline ) { map.serialize_entry("underlined"    , &true)?; }
        if (self.strike    ) { map.serialize_entry("strikethrough" , &true)?; }
        if (self.obfuscate ) { map.serialize_entry("obfuscated"    , &true)?; }

        if let Some(colour) = &self.shadow {
            map.serialize_entry("shadow_color", &(
                ((colour.a as u32) << 24) | ((colour.r as u32) << 16) | ((colour.g as u32) << 8) | (colour.b as u32)
            ))?;
        }

        if let Some(insertion) = &self.insertion {
            map.serialize_entry("insertion", insertion)?;
        }

        if let Some(on_click) = &self.on_click {
            map.serialize_entry("click_event", on_click)?;
        }

        if let Some(tooltip) = &self.tooltip {
            map.serialize_entry("hover_event", &HoverEventTooltip(tooltip))?;
        }

        Ok(())
    }
}


struct HoverEventTooltip<'l>(&'l Text);

impl Ser for HoverEventTooltip<'_> {
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    {
        let mut map = serer.serialize_map(Some(2))?;
        map.serialize_entry("action", "show_text")?;
        map.serialize_entry("value", &self.0)?;
        map.end()
    }
}
