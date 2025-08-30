use super::{
    NbtSerError,
    NbtRootSerer, TagWrite,
    tag
};
use std::io::Write;
use serde::ser::{
    Serialize as Ser,
    SerializeSeq as SerSeq,
    SerializeTuple as SerTuple
};


pub(super) struct NbtSeqSerer<'l, W>
where
    W : Write
{
    writer : &'l mut W,
    len    : u32,
    first  : bool
}

impl<'l, W> NbtSeqSerer<'l, W>
where
    W : Write
{ pub(super) fn new(writer : &'l mut W, len : u32) -> Self { Self {
    writer, len, first : true
} } }


impl<'l, W> SerSeq for NbtSeqSerer<'l, W>
where
    W : Write
{
    type Ok    = ();
    type Error = NbtSerError;

    #[inline(always)]
    fn serialize_element<T>(&mut self, value : &T) -> Result<(), Self::Error>
    where
        T : ?Sized + Ser
    { SerTuple::serialize_element(self, value) }

    #[inline(always)]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerTuple::end(self)
    }

}


impl<'l, W> SerTuple for NbtSeqSerer<'l, W>
where
    W : Write
{
    type Ok    = ();
    type Error = NbtSerError;

    fn serialize_element<T>(&mut self, value : &T) -> Result<(), Self::Error>
    where
        T : ?Sized + Ser
    {
        let after_tag = if (self.first) { TagWrite::TagAndU32(self.len) } else { TagWrite::None };
        self.first = false;
        value.serialize(NbtRootSerer::new(&mut*self.writer, after_tag))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if (self.first) {
            self.writer.write_all(&[tag::END])?;
            self.writer.write_all(&0u32.to_be_bytes())?;
        }
        Ok(())
    }

}
