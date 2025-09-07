use super::{
    AccountProfile,
    AccountProperty,
    AccountPropertyKey
};
use core::fmt::{ self, Formatter };
use serde::{
    Deserialize as Deser,
    Deserializer as Deserer,
    de::{ self,
        MapAccess,
        SeqAccess,
        Visitor,
        IgnoredAny
    }
};


const FIELD_UUID       : &str = "id";
const FIELD_USERNAME   : &str = "name";
const FIELD_PROPERTIES : &str = "properties";

const PROFILE_FIELDS : &[&str] = &[
    FIELD_UUID,
    FIELD_USERNAME,
    FIELD_PROPERTIES
];


impl<'de> Deser<'de> for AccountProfile {
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    {
        deserer.deserialize_struct("AccountProfile", PROFILE_FIELDS, AccountProfileVisitor)
    }
}


struct AccountProfileVisitor;
impl<'de> Visitor<'de> for AccountProfileVisitor {
    type Value = AccountProfile;

    fn expecting(&self, f : &mut Formatter) -> fmt::Result {
        write!(f, "struct AccountProfile")
    }

    fn visit_map<A>(self, mut map : A) -> Result<Self::Value, A::Error>
    where
        A : MapAccess<'de>
    {
        let mut uuid       = None;
        let mut username   = None;
        let mut properties = Option::<AccountProperties>::None;
        while let Some(k) = map.next_key()? { match (k) {
            FIELD_UUID => {
                if (uuid.is_some()) {
                    return Err(de::Error::duplicate_field(FIELD_UUID));
                }
                uuid = Some(map.next_value()?);
            },
            FIELD_USERNAME => {
                if (username.is_some()) {
                    return Err(de::Error::duplicate_field(FIELD_USERNAME));
                }
                username = Some(map.next_value()?);
            },
            FIELD_PROPERTIES => {
                if (properties.is_some()) {
                    return Err(de::Error::duplicate_field(FIELD_PROPERTIES));
                }
                properties = Some(map.next_value()?);
            },
            _ => {
                map.next_value::<IgnoredAny>()?;
            }
        } }
        let properties = properties.unwrap_or_default();
        Ok(AccountProfile {
            uuid     : uuid.ok_or_else(|| de::Error::missing_field("id"))?,
            username : username.ok_or_else(|| de::Error::missing_field("name"))?,
            skin     : properties.skin
        })
    }

}


#[derive(Default)]
struct AccountProperties {
    skin : Option<AccountProperty>
}
impl<'de> Deser<'de> for AccountProperties {
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    {
        deserer.deserialize_seq(AccountPropertiesVisitor)
    }
}

struct AccountPropertiesVisitor;
impl<'de> Visitor<'de> for AccountPropertiesVisitor {
    type Value = AccountProperties;

    fn expecting(&self, f : &mut Formatter) -> fmt::Result {
        write!(f, "struct AccountProfile")
    }

    fn visit_seq<A>(self, mut seq : A) -> Result<Self::Value, A::Error>
    where
        A : SeqAccess<'de>
    {
        let mut skin = None;
        while let Some(v) = seq.next_element::<KeyedAccountProperty>()? {
            match (v.key) {
                AccountPropertyKey::Skin => {
                    if (skin.is_some()) {
                        return Err(de::Error::duplicate_field("textures"));
                    }
                    skin = Some(v.property);
                }
            }
        }
        Ok(AccountProperties {
            skin
        })
    }

}


#[derive(Deser)]
struct KeyedAccountProperty {
    #[serde(rename = "name")]
    key      : AccountPropertyKey,
    #[serde(flatten)]
    property : AccountProperty
}
