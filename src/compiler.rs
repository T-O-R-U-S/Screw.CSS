use lightningcss::properties::background::BackgroundClip;
use lightningcss::properties::Property;
use lightningcss::selector::Selector;
use lightningcss::vendor_prefix::VendorPrefix;
use phf::phf_map;
use smallvec::smallvec;

use crate::parser::Parameter;

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
    }
};

pub fn translate_function((name, parameter): (String, Parameter)) -> Option<Property<'static>> {
    SCREW_FUNCTIONS.get(&name).map(|function| function(parameter))
}