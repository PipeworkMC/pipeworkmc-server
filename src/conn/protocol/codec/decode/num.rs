use crate::conn::protocol::codec::decode::{
    PacketDecode,
    IncompleteDecodeError
};


macro impl_packetdecode_for_num($ty:ty) {
    impl PacketDecode for $ty {
        type Error = IncompleteDecodeError;

        fn decode(buf : &mut super::DecodeBuf<'_>)
            -> Result<Self, Self::Error>
        { Ok(Self::from_be_bytes(buf.read_n_const()?)) }
    }
}

impl_packetdecode_for_num!(u8);
impl_packetdecode_for_num!(i8);
impl_packetdecode_for_num!(u16);
impl_packetdecode_for_num!(i16);
impl_packetdecode_for_num!(u32);
impl_packetdecode_for_num!(i32);
impl_packetdecode_for_num!(u64);
impl_packetdecode_for_num!(i64);
impl_packetdecode_for_num!(u128);
impl_packetdecode_for_num!(i128);
impl_packetdecode_for_num!(f32);
impl_packetdecode_for_num!(f64);


impl PacketDecode for bool {
    type Error = IncompleteDecodeError;

    fn decode(buf : &mut super::DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(buf.read()? != 0) }
}
