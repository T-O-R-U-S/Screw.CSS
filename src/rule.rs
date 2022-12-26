use lightningcss::rules::CssRule;
use lightningcss::selector::Selector;
use lightningcss::media_query::MediaCondition;
use crate::tokens::FunCall;

#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash)]
pub enum ScreenSize {
  Xs = 320,
  Sm = 640,
  Md = 768,
  Lg = 1024,
  Xl = 1280,
  Xl2 = 1536,
  // 1440p
  Xl3 = 2560,
  // 4k
  Xl4 = 3840
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
  Px(u64),
  Pc(f64),
  Pt(f64),
  In(f64),
  Cm(f64),
  Mm(f64),
  Vh(f64),
  Vw(f64),
  VMin(f64),
  VMax(f64),
  Percent(f64),
  Em(f64),
  Rem(f64),
  /// Arbitrary units decided upon by Screw
  Arbitrary(f64)
}

#[derive(Debug, Clone, Copy)]
pub enum Colour {
  Hsv(f64, f64, f64),
  Hsl(f64, f64, f64),
  Hwb(f64, f64, f64),
  Lch(f64, f64, f64),
  Lab(f64, f64, f64),
  Rgb(f64, f64, f64),
  Hex(u8,  u8,  u8 ),
}

#[derive(Hash, Debug, Clone, Copy)]
// TODO: CSS keywords
pub enum Keyword {
  Hidden,
  Block,
  Grid,
  Flex,
  Inline,
  InlineFlex,
  InlineGrid,
  InlineBlock,
}

#[derive(Debug, Clone)]
pub enum Parameter {
  Size(Size),
  Colour(u8, u8, u8, f64),
  Keyword(Keyword),
  FunCall(FunCall)
}

pub enum Output<'a> {
  Selector(Selector<'a>),
  MediaCondition(MediaCondition<'a>),
  Rule(CssRule<'a>),
}

type ScrewFunction = fn(Vec<Parameter>) -> Output<'static>;

#[derive(Debug, Clone)]
pub struct SelectorRulePair {
  selector:  Vec<FunCall>,
  functions: Vec<FunCall>
}