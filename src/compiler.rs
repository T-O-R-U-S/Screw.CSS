use itertools::Itertools;
use lightningcss::properties::background::BackgroundClip;
use lightningcss::properties::Property;
use lightningcss::properties::size::Size;
use lightningcss::values::length::{LengthPercentage, LengthValue};
use lightningcss::values::percentage::DimensionPercentage;
use lightningcss::vendor_prefix::VendorPrefix;
use phf::phf_map;
use smallvec::smallvec;

use crate::parser::Parameter;
use crate::parser::Unit;

// TODO: Remove unwraps, make functions fallible
const SCREW_FUNCTIONS: phf::Map<&'static str, fn(Parameter) -> Property<'static>> = phf_map! {
    "bg-clip" => |parameter| {
        let mut bg_clip = smallvec![];

        match parameter {
            Parameter::Ident(id) => {
                bg_clip.push(match id.as_str() {
                    "text" => BackgroundClip::Text,
                    "padding" => BackgroundClip::PaddingBox,
                    "content" => BackgroundClip::ContentBox,
                    "border" => BackgroundClip::BorderBox,
                    any => panic!("Unknown keyword '{any}'")
                })
            },
            parameter => panic!("Unexpected {parameter:?}")
        }

        Property::BackgroundClip(bg_clip, VendorPrefix::all())
    },
    "h" => |parameter| {
        let length: LengthPercentage = parameter.try_into().unwrap();

        Property::Height(Size::LengthPercentage(length))
    },
    "w" => |parameter| {
        let length: LengthPercentage = parameter.try_into().unwrap();

        Property::Width(Size::LengthPercentage(length))
    }
};

pub fn translate_function((name, parameter): (String, Box<Parameter>)) -> Option<Property<'static>> {
    SCREW_FUNCTIONS.get(&name).map(|function| function(*parameter))
}

pub fn compile(inputs: Vec<Parameter>) -> Vec<Property<'static>> {
    inputs.into_iter().map(|x| translate_function(x.unwrap_function()).unwrap()).collect_vec()
}