use crate::data::ident::Ident;
use serde::ser::{
    Serialize as Ser,
    Serializer as Serer,
    SerializeMap as _
};


#[derive(Clone, Debug)]
pub struct ItemStack {
    pub id    : Ident,
    pub count : u32,
    // TODO: components
}

impl Ser for ItemStack {
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    {
        let mut map = serer.serialize_map(Some(2))?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("count", &self.count)?;
        map.end()
    }
}
