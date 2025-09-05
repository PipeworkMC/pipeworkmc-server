use crate::conn::protocol::codec::encode::{
    PacketEncode,
    EncodeBuf
};
use crate::data::{
    bounded_string::BoundedString,
    varint::VarInt
};
use bevy_ecs::component::Component;
use serde::Deserialize as Deser;
use uuid::Uuid;


mod deser;


#[derive(Clone, Component, Debug)]
pub struct AccountProfile {
    pub uuid     : Uuid,
    pub username : BoundedString<16>,
    pub skin     : Option<AccountProperty>
}

#[derive(Clone, Deser, Debug)]
pub struct AccountProperty {
    pub value : String,
    #[serde(rename = "signature")]
    pub sig   : Option<String>,
}


unsafe impl PacketEncode for AccountProfile {

    fn encode_len(&self) -> usize {
        self.uuid.encode_len()
        + self.username.encode_len()
        + KeyedAccountProperties {
            skin : KeyedAccountProperty { key : "textures", property : self.skin.as_ref() }
        }.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.uuid.encode(buf);
        self.username.encode(buf);
        KeyedAccountProperties {
            skin : KeyedAccountProperty { key : "textures", property : self.skin.as_ref() }
        }.encode(buf);
    } }

}


struct KeyedAccountProperties<'l> {
    skin : KeyedAccountProperty<'l>
}

unsafe impl PacketEncode for KeyedAccountProperties<'_> {

    fn encode_len(&self) -> usize {
        let len = if (self.skin.property.is_some()) { 1 } else { 0 };

        VarInt::<u32>(len as u32).encode_len()
        + self.skin.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        let len = if (self.skin.property.is_some()) { 1 } else { 0 };
        VarInt::<u32>(len as u32).encode(buf);
        self.skin.encode(buf);
    } }

}


struct KeyedAccountProperty<'l> {
    key      : &'static str,
    property : Option<&'l AccountProperty>
}

unsafe impl PacketEncode for KeyedAccountProperty<'_> {

    fn encode_len(&self) -> usize {
        if let Some(property) = self.property {
            self.key.encode_len()
            + property.value.encode_len()
            + property.sig.encode_len()
        } else { 0 }
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        if let Some(property) = self.property {
            self.key.encode(buf);
            property.value.encode(buf);
            property.sig.encode(buf);
        }
    } }

}


#[derive(Deser)]
enum AccountPropertyKey {
    #[serde(rename = "textures")]
    Skin
}
