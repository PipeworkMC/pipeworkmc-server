use super::{
    NbtSerError,
    never::Never
};
use serde::ser::{
    Serialize as Ser,
    Serializer as Serer
};


#[derive(Default)]
pub(super) struct NbtStrSerer {
    pub(super) string : String
}

impl Serer for &mut NbtStrSerer {
    type Ok    = ();
    type Error = NbtSerError;

    type SerializeSeq           = Never;
    type SerializeTuple         = Never;
    type SerializeTupleStruct   = Never;
    type SerializeTupleVariant  = Never;
    type SerializeMap           = Never;
    type SerializeStruct        = Never;
    type SerializeStructVariant = Never;

    fn serialize_bool(self, _ : bool) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_i8(self, _ : i8) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_i16(self, _ : i16) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_i32(self, _ : i32) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_i64(self, _ : i64) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_u8(self, _ : u8) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_u16(self, _ : u16) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_u32(self, _ : u32) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_u64(self, _ : u64) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_f32(self, _ : f32) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_f64(self, _ : f64) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_char(self, _ : char) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_str(self, s : &str) -> Result<Self::Ok, Self::Error> {
        self.string = s.to_string();
        Ok(())
    }
    fn serialize_bytes(self, _ : &[u8]) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_some<T>(self, _ : &T) -> Result<Self::Ok, Self::Error> where T : ?Sized + Ser { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_unit_struct(self, _ : &'static str) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_unit_variant(self, _ : &'static str, _ : u32, _ : &'static str) -> Result<Self::Ok, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_newtype_struct<T>(self, _ : &'static str, _ : &T) -> Result<Self::Ok, Self::Error> where T : ?Sized + Ser { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_newtype_variant<T>(self, _ : &'static str, _ : u32, _ : &'static str, _ : &T) -> Result<Self::Ok, Self::Error> where T : ?Sized + Ser { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_seq(self, _ : Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_tuple(self, _ : usize) -> Result<Self::SerializeTuple, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_tuple_struct(self, _ : &'static str, _ : usize) -> Result<Self::SerializeTupleStruct, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_tuple_variant(self, _ : &'static str, _ : u32, _ : &'static str, _ : usize) -> Result<Self::SerializeTupleVariant, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_map(self, _ : Option<usize>) -> Result<Self::SerializeMap, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_struct(self, _ : &'static str, _ : usize) -> Result<Self::SerializeStruct, Self::Error> { panic!("NBT map keys must be of type `&str`"); }
    fn serialize_struct_variant(self, _ : &'static str, _ : u32, _ : &'static str, _ : usize) -> Result<Self::SerializeStructVariant, Self::Error> { panic!("NBT map keys must be of type `&str`"); }

}
