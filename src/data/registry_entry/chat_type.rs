use super::RegistryEntryType;
use crate::data::{
    ident::Ident,
    nbt::to_network as to_network_nbt,
    text::TextStyle
};
use std::{
    borrow::Cow,
    io::Write
};
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct ChatTypeRegistryEntry<'l> {
    pub chat      : ChatTypeDecoration<'l>,
    pub narration : ChatTypeDecoration<'l>
}

#[derive(Ser, Debug)]
pub struct ChatTypeDecoration<'l> {
    pub translation_key : Cow<'l, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style           : Option<TextStyle>,
    pub parameters      : Cow<'l, [ChatTypeDecoParam]>
}

#[derive(Clone, Copy, Ser, Debug)]
pub enum ChatTypeDecoParam {
    #[serde(rename = "sender")]
    Sender,
    #[serde(rename = "target")]
    Target,
    #[serde(rename = "content")]
    Content
}


impl RegistryEntryType for ChatTypeRegistryEntry<'_> {
    const REGISTRY_ID : Ident = Ident::new("minecraft:chat_type");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
