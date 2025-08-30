use std::borrow::Cow;
use serde::ser::{
    Serialize as Ser,
    Serializer as Serer
};


#[derive(Clone, Debug)] // TODO: String-like debug formatter
pub struct Ident {
    namespace : Cow<'static, str>,
    path      : Cow<'static, str>
}

impl Ser for Ident {
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    { serer.serialize_str(&format!("{}:{}", self.namespace, self.path)) }
}
