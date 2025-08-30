use super::NbtSerError;
use serde::ser::{
    Serialize as Ser,
    SerializeSeq as SerSeq,
    SerializeTuple as SerTuple,
    SerializeTupleStruct as SerTupleStruct,
    SerializeTupleVariant as SerTupleVariant,
    SerializeMap as SerMap,
    SerializeStruct as SerStruct,
    SerializeStructVariant as SerStructVariant,
    Serializer as Serer
};


pub enum Never { }

impl Serer for Never {
    type Ok    = ();
    type Error = NbtSerError;
    type SerializeSeq           = Never;
    type SerializeTuple         = Never;
    type SerializeTupleStruct   = Never;
    type SerializeTupleVariant  = Never;
    type SerializeMap           = Never;
    type SerializeStruct        = Never;
    type SerializeStructVariant = Never;
    fn serialize_bool(self, _ : bool) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_i8(self, _ : i8) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_i16(self, _ : i16) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_i32(self, _ : i32) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_i64(self, _ : i64) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_u8(self, _ : u8) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_u16(self, _ : u16) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_u32(self, _ : u32) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_u64(self, _ : u64) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_f32(self, _ : f32) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_f64(self, _ : f64) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_char(self, _ : char) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_str(self, _ : &str) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_bytes(self, _ : &[u8]) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_some<T>(self, _ : &T) -> Result<Self::Ok, Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_unit_struct(self, _ : &'static str) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_unit_variant(self, _ : &'static str, _ : u32, _ : &'static str) -> Result<Self::Ok, Self::Error> { unreachable!() }
    fn serialize_newtype_struct<T>(self, _ : &'static str, _ : &T) -> Result<Self::Ok, Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn serialize_newtype_variant<T>(self, _ : &'static str, _ : u32, _ : &'static str, _ : &T) -> Result<Self::Ok, Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn serialize_seq(self, _ : Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { unreachable!() }
    fn serialize_tuple(self, _ : usize) -> Result<Self::SerializeTuple, Self::Error> { unreachable!() }
    fn serialize_tuple_struct(self, _ : &'static str, _ : usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unreachable!() }
    fn serialize_tuple_variant(self, _ : &'static str, _ : u32, _ : &'static str, _ : usize) -> Result<Self::SerializeTupleVariant, Self::Error> { unreachable!() }
    fn serialize_map(self, _ : Option<usize>) -> Result<Self::SerializeMap, Self::Error> { unreachable!() }
    fn serialize_struct(self, _ : &'static str, _ : usize) -> Result<Self::SerializeStruct, Self::Error> { unreachable!() }
    fn serialize_struct_variant(self, _ : &'static str, _ : u32, _ : &'static str, _ : usize) -> Result<Self::SerializeStructVariant, Self::Error> { unreachable!() }
}

impl SerSeq for Never {
    type Ok    = ();
    type Error = NbtSerError;
    fn serialize_element<T>(&mut self, _ : &T) -> Result<(), Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl SerTuple for Never {
    type Ok    = ();
    type Error = NbtSerError;
    fn serialize_element<T>(&mut self, _ : &T) -> Result<(), Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl SerTupleStruct for Never {
    type Ok    = ();
    type Error = NbtSerError;
    fn serialize_field<T>(&mut self, _ : &T) -> Result<(), Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl SerTupleVariant for Never {
    type Ok    = ();
    type Error = NbtSerError;
    fn serialize_field<T>(&mut self, _ : &T) -> Result<(), Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl SerMap for Never {
    type Ok    = ();
    type Error = NbtSerError;
    fn serialize_key<T>(&mut self, _ : &T) -> Result<(), Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn serialize_value<T>(&mut self, _ : &T) -> Result<(), Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl SerStruct for Never {
    type Ok    = ();
    type Error = NbtSerError;
    fn serialize_field<T>(&mut self, _ : &'static str, _ : &T) -> Result<(), Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}

impl SerStructVariant for Never {
    type Ok    = ();
    type Error = NbtSerError;
    fn serialize_field<T>(&mut self, _ : &'static str, _ : &T) -> Result<(), Self::Error> where T : ?Sized + Ser { unreachable!() }
    fn end(self) -> Result<Self::Ok, Self::Error> { unreachable!() }
}
