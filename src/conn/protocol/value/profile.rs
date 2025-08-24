use crate::conn::protocol::{
    codec::encode::{
        PacketEncode,
        EncodeBuf
    },
    value::bounded_string::BoundedString
};
use std::borrow::Cow;
use bevy_ecs::component::Component;
use serde::Deserialize as Deser;
use uuid::Uuid;


#[derive(Clone, Component, Deser, Debug)]
pub struct Profile {
    #[serde(rename = "id")]
    pub uuid     : Uuid,
    #[serde(rename = "name")]
    pub username : BoundedString<16>,
    #[serde(rename = "properties", default = "empty_cow_slice")]
    pub props    : Cow<'static, [ProfileProperty]>
}
fn empty_cow_slice() -> Cow<'static, [ProfileProperty]> { Cow::Borrowed(&[]) }

#[derive(Clone, Deser, Debug)]
pub struct ProfileProperty {
    #[serde(rename = "name")]
    pub key   : ProfilePropertyKey,
    pub value : String,
    #[serde(rename = "signature")]
    pub sig   : Option<String>,
}

#[derive(Clone, Deser, Debug)]
pub enum ProfilePropertyKey {
    #[serde(rename = "textures")]
    Textures
}

impl ProfilePropertyKey {
    pub fn as_str(&self) -> &'static str { match (self) {
        Self::Textures => "textures"
    } }
}


unsafe impl PacketEncode for Profile {

    fn encode_len(&self) -> usize {
        self.uuid.encode_len()
        + self.username.encode_len()
        + self.props.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.uuid.encode(buf);
        self.username.encode(buf);
        self.props.encode(buf);
    } }

}

unsafe impl PacketEncode for ProfileProperty {

    fn encode_len(&self) -> usize {
        self.key.as_str().encode_len()
        + self.value.encode_len()
        + self.sig.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.key.as_str().encode(buf);
        self.value.encode(buf);
        self.sig.encode(buf);
    } }

}
