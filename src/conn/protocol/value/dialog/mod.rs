use crate::conn::protocol::value::{
    action::Action,
    itemstack::ItemStack,
    text::Text
};
use crate::util::slice_is_empty;
use std::borrow::Cow;
use serde::Serialize as Ser;


#[derive(Clone, Ser, Debug)]
pub struct Dialog {
    #[serde(flatten)]
    pub kind      : DialogKind,
    pub title     : Text,
    #[serde(skip_serializing_if = "slice_is_empty")]
    pub body      : Cow<'static, [DialogBody]>,
    #[serde(skip_serializing_if = "slice_is_empty")]
    pub inputs    : Cow<'static, [DialogInput]>,
    #[serde(rename = "can_close_with_escape")]
    pub escapable : bool,
    // pub pause     : bool, // Not applicable on multiplayer servers.
    #[serde(rename = "after_action")]
    pub after     : DialogAfterAction
}

#[derive(Clone, Ser, Debug)]
#[serde(tag = "type")]
pub enum DialogKind {
    #[serde(rename = "minecraft:notice")]
    Notice {
        #[serde(skip_serializing_if = "Option::is_none")]
        action : Option<DialogButton>
    },
    #[serde(rename = "minecraft:confirmation")]
    Confirmation {
        yes : DialogButton,
        no  : DialogButton
    },
    #[serde(rename = "minecraft:multi_action")]
    MultiAction {
        actions : Cow<'static, [DialogButton]>,
        #[serde(skip_serializing_if = "Option::is_none")]
        columns : Option<u32>,
        #[serde(rename = "exit_action", skip_serializing_if = "Option::is_none")]
        exit    : Option<DialogButton>
    },
    #[serde(rename = "minecraft:server_links")]
    ServerLinks {
        #[serde(rename = "exit_action", skip_serializing_if = "Option::is_none")]
        exit         : Option<DialogButton>,
        #[serde(skip_serializing_if = "Option::is_none")]
        columns      : Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        button_width : Option<u32>
    },
    #[serde(rename = "minecraft:dialog_list")]
    DialogList {
        dialogs      : Cow<'static, [Dialog]>,
        #[serde(rename = "exit_action", skip_serializing_if = "Option::is_none")]
        exit         : Option<DialogButton>,
        #[serde(skip_serializing_if = "Option::is_none")]
        columns      : Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        button_width : Option<u32>
    }
}

#[derive(Clone, Ser, Debug)]
pub struct DialogButton {
    pub label   : Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip : Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width   : Option<u32>,
    pub action  : Action
}

#[derive(Clone, Ser, Debug)]
#[serde(tag = "type")]
pub enum DialogBody {
    #[serde(rename = "minecraft:plain_message")]
    Plain {
        contents : Text,
        #[serde(skip_serializing_if = "Option::is_none")]
        width    : Option<u32>
    },
    #[serde(rename = "minecraft:item")]
    Item {
        stack       : ItemStack,
        description : DialogItemBodyDesc,
        #[serde(rename = "show_decoration")]
        decoration  : bool,
        #[serde(rename = "show_tooltip")]
        tooltip     : bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        width       : Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height      : Option<u32>
    }
}

#[derive(Clone, Ser, Debug)]
pub struct DialogItemBodyDesc {
    pub contents : Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width    : Option<u32>
}

#[derive(Clone, Ser, Debug)]
pub struct DialogInput {
    #[serde(flatten)]
    pub kind  : DialogInputKind,
    pub key   : String,
    pub label : Text
}

#[derive(Clone, Ser, Debug)]
#[serde(tag = "type")]
pub enum DialogInputKind {
    #[serde(rename = "minecraft:text")]
    Text {
        #[serde(skip_serializing_if = "Option::is_none")]
        width         : Option<u32>,
        label_visible : bool,
        initial       : Cow<'static, str>,
        #[serde(rename = "max_length")]
        max_len       : u32,
        multiline     : DialogTextInputMultiline
    },
    #[serde(rename = "minecraft:boolean")]
    Boolean {
        initial  : bool,
        on_true  : Cow<'static, str>,
        on_false : Cow<'static, str>
    },
    #[serde(rename = "minecraft:single_option")]
    SingleOption {
        label_visible : bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        width         : Option<u32>,
        options       : Cow<'static, [DialogInputOption]>
    },
    #[serde(rename = "minecraft:number_range")]
    NumberRange {
        label_format : Cow<'static, str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width        : Option<u32>,
        #[serde(rename = "start")]
        min          : u32,
        #[serde(rename = "end")]
        max          : u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        step         : Option<u32>,
        initial      : u32
    }
}

#[derive(Clone, Ser, Debug)]
pub struct DialogTextInputMultiline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines : Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height    : Option<u32>
}

#[derive(Clone, Ser, Debug)]
pub struct DialogInputOption {
    pub id      : String,
    pub display : Text,
    pub initial : bool
}

#[derive(Clone, Ser, Debug)]
pub enum DialogAfterAction {
    #[serde(rename = "close")]
    Close,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "wait_for_response")]
    WaitForResponse
}
