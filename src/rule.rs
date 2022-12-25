use std::convert::Into;

pub struct ArbitraryUnit(i64);

pub enum Size {
  Centimeter(f64),
  Millimeter(f64),
  Inch(f64),
  Point(f64),
  Pica(f64),
  Pixel(u64),
  Fraction(i64, u64),
  Percentage(f64),
  Vw(f64),
  Vh(f64),
  VMax(f64),
  VMin(f64),
  Rem(f64),
  Em(f64),
  Ex(f64),
  Ch(f64),
  /// Arbitrary size unit
  Arbitrary(i64)
}

#[repr(u64)]
/// Size, measured in pixels
pub enum ScreenSize {
  Xs = 480,
  Sm = 640,
  Md = 768,
  Lg = 1024,
  // 720p
  Xl = 1280,
  Xl2 = 1536,
  // 1440p
  Xl3 = 2560,
  // 4K
  Xl4 = 3840,
  Custom(u64)
}

#[derive(Debug, Copy, Clone, Hash)]
pub enum Selector {
  Size(ScreenSize),
  Compatible(Rule),
  Dark,
}

pub enum CssPositionable {
  Padding,
  Margin,
  Outline,
  Ring,
  Border,
  Background,
  BoxShadow
}

pub enum Flex {
  Grow,
  Row(bool),
  Col(bool),
  Wrap(bool),

}

#[repr(u32)]
pub enum Colour {
  White = [255, 255, 255, 255].into(),
  Black = [  0,   0,   0, 255].into()
}

#[derive(Debug, Copy, Clone, Hash)]
pub enum Rule {
  Display(Display),
  Width(Size),
  Height(Size),
  XyValues {
    kind: CssPositionable,
    top: Size,
    bottom: Size,
    left: Size,
    right: Size
  },
  // 8-bit RGBA colours
  Colour(CssPositionable, u32),
  Opacity(CssPositionable, f32),

}

#[derive(Debug, Copy, Clone, Hash)]
pub enum Display {
  Hidden,
  Inline,
  Block,
  Grid,
  Flex,
  FlowRoot,
  InlineBlock,
  InlineGrid,
  InlineFlex,
}

pub struct SelectorRulePair {
  selector: Vec<Selector>,
  rule: Vec<Rule>
}