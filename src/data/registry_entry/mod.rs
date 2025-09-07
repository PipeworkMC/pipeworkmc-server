use crate::data::{
    ident::Ident,
    nbt::to_network as to_network_nbt
};
use core::fmt::Debug;
use std::io::Write;
use serde::Serialize as Ser;


pub mod banner_pattern;
pub mod cat_variant;
pub mod chat_type;
pub mod chicken_variant;
pub mod cow_variant;
pub mod damage_type;
// TODO: dialog
pub mod dimension_type;
pub mod frog_variant;
pub mod trim_material;
pub mod painting_variant;
pub mod pig_variant;
pub mod trim_pattern;
pub mod wolf_variant;
pub mod wolf_sound_variant;

pub mod worldgen;


pub trait RegistryEntryType
where
    Self : Debug + Ser
{
    const REGISTRY_ID : Ident;

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write;
}


impl<T> RegistryEntryType for &T
where
    T : RegistryEntryType
{
    const REGISTRY_ID : Ident = <T as RegistryEntryType>::REGISTRY_ID;

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}


impl<T> RegistryEntryType for Option<T>
where
    T : RegistryEntryType
{
    const REGISTRY_ID : Ident = <T as RegistryEntryType>::REGISTRY_ID;

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    { self.as_ref().is_some_and(|inner| T::to_network_nbt(inner, writer)) }
}
