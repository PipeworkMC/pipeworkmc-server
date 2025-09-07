use super::{
    NbtSerError,
    NbtSeqSerer,
    NbtMapSerer,
    never::Never,
    tag
};
use std::io::{ self, Write };
use cesu8::to_java_cesu8;
use serde::ser::{
    Serialize as Ser,
    Serializer as Serer
};


pub(super) struct NbtRootSerer<'l, 'k, W>
where
    W : Write
{
    writer    : &'l mut W,
    tag_write : TagWrite<'k>
}

impl<'l, 'k, W> NbtRootSerer<'l, 'k, W>
where
    W : Write
{

    pub(super) fn new(writer : &'l mut W, tag_write : TagWrite<'k>) -> Self { Self {
        writer, tag_write
    } }

    fn handle_tag_write(self : &mut &mut Self, tag : u8) -> io::Result<()> { match (&self.tag_write) {
        TagWrite::None => Ok(()),
        TagWrite::Tag => {
            self.writer.write_all(&[tag])?;
            Ok(())
        },
        TagWrite::TagAndU32(v) => {
            self.writer.write_all(&[tag])?;
            self.writer.write_all(&v.to_be_bytes())?;
            Ok(())
        },
        TagWrite::TagAndString(s) => {
            self.writer.write_all(&[tag])?;
            let jstring = to_java_cesu8(s);
            self.writer.write_all(&u16::try_from(jstring.len()).unwrap().to_be_bytes())?;
            self.writer.write_all(&jstring)?;
            Ok(())
        }
    } }

}


pub(super) enum TagWrite<'l> {
    None,
    Tag,
    TagAndU32(u32),
    TagAndString(&'l str)
}


impl<'l, W> Serer for NbtRootSerer<'l, '_, W>
where
    W : Write
{
    type Ok    = ();
    type Error = NbtSerError;

    type SerializeSeq           = NbtSeqSerer<'l, W>;
    type SerializeTuple         = NbtSeqSerer<'l, W>;
    type SerializeTupleStruct   = Never;
    type SerializeTupleVariant  = Never;
    type SerializeMap           = NbtMapSerer<'l, W>;
    type SerializeStruct        = NbtMapSerer<'l, W>;
    type SerializeStructVariant = Never;

    #[inline]
    fn serialize_bool(self, v : bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(if (v) { 1u8 } else { 0u8 })
    }
    #[inline]
    fn serialize_i8(self, v : i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(v.cast_unsigned())
    }
    #[inline]
    fn serialize_i16(self, v : i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u16(v.cast_unsigned())
    }
    #[inline]
    fn serialize_i32(self, v : i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(v.cast_unsigned())
    }
    #[inline]
    fn serialize_i64(self, v : i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.cast_unsigned())
    }
    fn serialize_u8(mut self, v : u8) -> Result<Self::Ok, Self::Error> {
        (&mut self).handle_tag_write(tag::BYTE)?;
        self.writer.write_all(&[v])?;
        Ok(())
    }
    fn serialize_u16(mut self, v : u16) -> Result<Self::Ok, Self::Error> {
        (&mut self).handle_tag_write(tag::SHORT)?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }
    fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
        (&mut self).handle_tag_write(tag::INT)?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }
    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        (&mut self).handle_tag_write(tag::LONG)?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }
    fn serialize_f32(mut self, v : f32) -> Result<Self::Ok, Self::Error> {
        (&mut self).handle_tag_write(tag::FLOAT)?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }
    fn serialize_f64(mut self, v : f64) -> Result<Self::Ok, Self::Error> {
        (&mut self).handle_tag_write(tag::DOUBLE)?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v : char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; 4];
        let     s   = v.encode_utf8(&mut buf);
        self.serialize_str(s)
    }
    fn serialize_str(mut self, v : &str) -> Result<Self::Ok, Self::Error> {
        (&mut self).handle_tag_write(tag::STRING)?;
        let jstring = to_java_cesu8(v);
        self.writer.write_all(&u16::try_from(jstring.len()).unwrap().to_be_bytes())?;
        self.writer.write_all(&jstring)?;
        Ok(())
    }
    fn serialize_bytes(self, _ : &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!() // TODO: BArray
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        panic!("NBT serialiser does not support `Option`");
    }
    fn serialize_some<T>(self, _ : &T) -> Result<Self::Ok, Self::Error>
    where
        T : ?Sized + Ser
    { panic!("NBT serialiser does not support `Option`"); }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        panic!("NBT serialiser does not support `()`");
    }

    fn serialize_unit_struct(self, _ : &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    #[inline(always)]
    fn serialize_unit_variant(
        self,
        _name          : &'static str,
        _variant_index : u32,
        variant        : &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        variant.serialize(self)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name  : &'static str,
        _value : &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Ser {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name          : &'static str,
        _variant_index : u32,
        _variant       : &'static str,
        _value         : &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Ser {
        todo!()
    }

    fn serialize_seq(self, len : Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let Some(len) = len else { panic!("NBT sequence serialiser requires length to be known in advance"); };
        self.serialize_tuple(len)
    }

    fn serialize_tuple(mut self, len : usize) -> Result<Self::SerializeTuple, Self::Error> {
        (&mut self).handle_tag_write(tag::LIST)?; // TODO: BArray, IArray, LArray
        Ok(NbtSeqSerer::new(&mut*self.writer, len.try_into().unwrap()))
    }

    fn serialize_tuple_struct(
        self,
        _name : &'static str,
        _len  : usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name          : &'static str,
        _variant_index : u32,
        _variant       : &'static str,
        _len           : usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(mut self, _ : Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        (&mut self).handle_tag_write(tag::COMPOUND)?;
        Ok(NbtMapSerer::from(&mut*self.writer))
    }

    fn serialize_struct(
        mut self,
        _name : &'static str,
        _len  : usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        (&mut self).handle_tag_write(tag::COMPOUND)?;
        Ok(NbtMapSerer::from(&mut*self.writer))
    }

    fn serialize_struct_variant(
        self,
        _name          : &'static str,
        _variant_index : u32,
        _variant       : &'static str,
        _len           : usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

}
