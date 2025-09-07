use super::{
    NbtSerError,
    NbtRootSerer, TagWrite,
    NbtStrSerer,
    tag
};
use std::io::Write;
use serde::ser::{
    Serialize as Ser,
    SerializeMap as SerMap,
    SerializeStruct as SerStruct
};


pub(super) struct NbtMapSerer<'l, W>
where
    W : Write
{
    writer : &'l mut W
}

impl<'l, W> From<&'l mut W> for NbtMapSerer<'l, W>
where
    W : Write
{ fn from(writer : &'l mut W) -> Self { Self {
    writer
} } }


impl<'l, W> SerMap for NbtMapSerer<'l, W>
where
    W : Write
{
    type Ok    = ();
    type Error = NbtSerError;

    #[inline(always)]
    fn serialize_key<T>(&mut self, _ : &T) -> Result<(), Self::Error> where T : ?Sized + Ser { unreachable!() }
    #[inline(always)]
    fn serialize_value<T>(&mut self, _ : &T) -> Result<(), Self::Error> where T : ?Sized + Ser { unreachable!() }

    fn serialize_entry<K, V>(&mut self, key : &K, value : &V) -> Result<(), Self::Error>
    where
        K : ?Sized + Ser,
        V : ?Sized + Ser
    {
        let mut key_ser = NbtStrSerer::default();
        key.serialize(&mut key_ser)?;
        value.serialize(NbtRootSerer::new(self.writer, TagWrite::TagAndString(&key_ser.string)))?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[tag::END])?;
        Ok(())
    }

}


impl<'l, W> SerStruct for NbtMapSerer<'l, W>
where
    W : Write
{
    type Ok    = ();
    type Error = NbtSerError;

    #[inline]
    fn serialize_field<T>(&mut self, key : &'static str, value : &T) -> Result<(), Self::Error>
    where
        T : ?Sized + Ser
    { value.serialize(NbtRootSerer::new(self.writer, TagWrite::TagAndString(key))) }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[tag::END])?;
        Ok(())
    }
}
