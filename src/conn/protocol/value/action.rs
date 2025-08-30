use crate::conn::protocol::value::{
    dialog::Dialog,
    ident::Ident
};
use std::borrow::Cow;
use serde::ser::{
    Serialize as Ser,
    Serializer as Serer,
    SerializeMap as _
};


#[derive(Clone, Debug)]
pub enum Action {
    OpenUrl {
        url : Cow<'static, str>
    },
    RunCommand {
        command : Cow<'static, str>
    },
    SuggestCommand {
        command : Cow<'static, str>
    },
    SetBookPage {
        page : u32
    },
    SetClipboard {
        text : Cow<'static, str>
    },
    ShowDialog {
        dialog : Box<Dialog>
    },
    Custom {
        id      : Ident,
        payload : String
    }
}

impl Ser for Action {
    fn serialize<S>(&self, serer : S) -> Result<S::Ok, S::Error>
    where
        S : Serer
    {
        let mut map = serer.serialize_map(Some(2))?;
        match (self) {
            Action::OpenUrl { url } => {
                map.serialize_entry("action", "open_url")?;
                map.serialize_entry("url", url)?;
            },
            Action::RunCommand { command } => {
                map.serialize_entry("action", "run_command")?;
                map.serialize_entry("command", command)?;
            },
            Action::SuggestCommand { command } => {
                map.serialize_entry("action", "suggest_command")?;
                map.serialize_entry("command", command)?;
            },
            Action::SetBookPage { page } => {
                map.serialize_entry("action", "change_page")?;
                map.serialize_entry("page", page)?;
            },
            Action::SetClipboard { text } => {
                map.serialize_entry("action", "copy_to_clipboard")?;
                map.serialize_entry("value", text)?;
            },
            Action::ShowDialog { dialog } => {
                map.serialize_entry("action", "show_dialog")?;
                map.serialize_entry("value", dialog)?;
            },
            Action::Custom { id, payload } => {
                map.serialize_entry("action", "custom")?;
                map.serialize_entry("id", id)?;
                map.serialize_entry("payload", payload)?;
            },
        }
        map.end()
    }
}
