use crate::data::box_cow::BoxCow;
use std::borrow::Cow;
use serde::Serialize as Ser;


#[derive(Clone, Ser, Debug)]
#[serde(tag = "type")]
pub enum IntProvider<'l, T>
where
    T : Clone
{

    #[serde(rename = "constant")]
    Constant {
        value : T
    },

    #[serde(rename = "uniform")]
    Uniform {
        min_inclusive : T,
        max_inclusive : T
    },

    #[serde(rename = "biased_to_bottom")]
    BiasedToBottom {
        min_inclusive : T,
        max_inclusive : T
    },

    #[serde(rename = "clamped")]
    Clamped {
        min_inclusive : T,
        max_inclusive : T,
        source        : BoxCow<'l, IntProvider<'l, T>>
    },

    #[serde(rename = "clamped_normal")]
    ClampedNormal {
        mean          : f32,
        deviation     : f32,
        min_inclusive : T,
        max_inclusive : T
    },

    #[serde(rename = "weighted_list")]
    WeightedList {
        distribution : Cow<'l, [WeightedProvider<'l, T>]>
    }

}


#[derive(Clone, Ser, Debug)]
pub struct WeightedProvider<'l, T>
where
    T : Clone
{
    pub data   : IntProvider<'l, T>,
    pub weight : u32
}
