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
use crate::data::ident::{
    Ident,
    IdentDecodeError
};
use core::fmt::{ self, Display, Formatter };
use std::borrow::Cow;


const CHANNEL_BRAND : Ident = Ident::new("minecraft:brand");


#[derive(Debug)]
pub enum ChannelData<'l> {
    Brand {
        brand : Cow<'l, str>
    },
    Custom {
        channel : Ident,
        data    : Cow<'l, [u8]>
    }
}


impl PacketDecode for ChannelData<'_> {
    type Error = ChannelDataDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let channel = Ident::decode(buf).map_err(ChannelDataDecodeError::Channel)?;
        Ok(if (channel == CHANNEL_BRAND) {
            Self::Brand { brand : Cow::Owned(<_>::decode(buf).map_err(ChannelDataDecodeError::Brand)?) }
        } else {
            Self::Custom { channel, data : Cow::Owned(buf.read_remaining().to_vec()) }
        })
    }
}

unsafe impl PacketEncode for ChannelData<'_> {

    fn encode_len(&self) -> usize { match (self) {
        Self::Brand { brand } => {
            CHANNEL_BRAND.encode_len()
            + brand.encode_len()
        },
        Self::Custom { channel, data } => {
            channel.encode_len()
            + data.len()
        }
    } }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::Brand { brand } => {
            CHANNEL_BRAND.encode(buf);
            brand.encode(buf);
        },
        Self::Custom { channel, data } => {
            channel.encode(buf);
            buf.write_slice(data);
        }
    } } }

}


#[derive(Debug)]
pub enum ChannelDataDecodeError {
    Channel(IdentDecodeError),
    Brand(StringDecodeError)
}

impl Display for ChannelDataDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Channel(err) => write!(f, "channel {err}"),
        Self::Brand(err)   => write!(f, "brand {err}")
    } }
}
