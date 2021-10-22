use crate::color::{
    xyY, CIEColor, Cmyk, Color, Hsl, Hsv, Illuminant, Lab, LchAB, LchUV, Luv, RgbWorkingSpace, Xyz,
};

use anyhow::{Error, Result};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, space0},
    combinator::map,
    error::{ErrorKind, ParseError},
    sequence::{delimited, preceded},
    Err, IResult, Parser,
};
use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
pub struct ColorFormat<'a>(Vec<FormatToken<'a>>);

impl<'a> ColorFormat<'a> {
    pub fn parse(text: &'a str) -> Result<ColorFormat<'a>> {
        match parse_color_format(text).map(|(_, fmt)| fmt) {
            Ok(fmt) => Ok(fmt),
            Err(e) => Err(Error::msg(format!("failed to parse color format - {}", e))),
        }
    }

    pub fn format_color(
        &self,
        color: &Color,
        ws: RgbWorkingSpace,
        illuminant: Illuminant,
    ) -> String {
        use ColorSymbol::*;

        let mut stack = LinkedList::new();
        let rgb = color.rgb();
        let cmyk = Cmyk::from(rgb);
        let hsl = Hsl::from(rgb);
        let hsv = Hsv::from(rgb);
        let xyz = Xyz::from_rgb(rgb, ws);
        let xyy = xyY::from(xyz);
        let lab = Lab::from_xyz(xyz, illuminant);
        let luv = Luv::from(xyz);
        let lch_ab = LchAB::from(lab);
        let lch_uv = LchUV::from(luv);

        let mut s = String::new();

        for token in &self.0 {
            match &token {
                FormatToken::Text(text) => s.push_str(text),
                FormatToken::Color(ColorField {
                    symbol,
                    digit_format,
                }) => match symbol {
                    Red | Green | Blue | Cyan | Magenta | Yellow | Key | Cyan100 | Magenta100
                    | Yellow100 | Key100 | HSLHue | HSLSaturation | HSLLight | HSVHue
                    | HSVSaturation | HSVValue | LabL | LabA | LabB | LCHabL | LCHabC | LCHabH
                    | LuvL | LuvU | LuvV | LCHuvL | LCHuvC | LCHuvH | xyYx | xyYy | xyYY | XYZx
                    | XYZy | XYZz | HSLHue360 | HSLSaturation100 | HSLLight100 | HSVHue360
                    | HSVSaturation100 | HSVValue100 => {
                        let num = match symbol {
                            Red => rgb.r(),
                            Green => rgb.g(),
                            Blue => rgb.b(),

                            Cyan => cmyk.c(),
                            Magenta => cmyk.m(),
                            Yellow => cmyk.y(),
                            Key => cmyk.k(),

                            Cyan100 => cmyk.c_scaled(),
                            Magenta100 => cmyk.m_scaled(),
                            Yellow100 => cmyk.y_scaled(),
                            Key100 => cmyk.k_scaled(),

                            HSLHue => hsl.h(),
                            HSLSaturation => hsl.s(),
                            HSLLight => hsl.l(),

                            HSLHue360 => hsl.h_scaled(),
                            HSLSaturation100 => hsl.s_scaled(),
                            HSLLight100 => hsl.l_scaled(),

                            HSVHue => hsv.h(),
                            HSVSaturation => hsv.s(),
                            HSVValue => hsv.v(),

                            HSVHue360 => hsv.h_scaled(),
                            HSVSaturation100 => hsv.s_scaled(),
                            HSVValue100 => hsv.v_scaled(),

                            LabL => lab.l(),
                            LabA => lab.a(),
                            LabB => lab.b(),

                            LCHabL => lch_ab.l(),
                            LCHabC => lch_ab.c(),
                            LCHabH => lch_ab.h(),

                            LuvL => luv.l(),
                            LuvU => luv.u(),
                            LuvV => luv.v(),

                            LCHuvL => lch_uv.l(),
                            LCHuvC => lch_uv.c(),
                            LCHuvH => lch_uv.h(),

                            xyYx => xyy.x(),
                            xyYy => xyy.y(),
                            xyYY => xyy.yy(),

                            XYZx => xyz.x(),
                            XYZy => xyz.y(),
                            XYZz => xyz.z(),
                            _ => unreachable!(),
                        };

                        DigitFormat::format_float(num, &mut s, &mut stack)
                    }
                    Red255 | Green255 | Blue255 => {
                        let num = match symbol {
                            Red255 => rgb.r_scaled(),
                            Green255 => rgb.g_scaled(),
                            Blue255 => rgb.b_scaled(),
                            _ => unreachable!(),
                        } as u32;

                        digit_format.format_num(num, &mut s, &mut stack);
                    }
                },
            }
        }

        s
    }
}

impl<'a> From<Vec<FormatToken<'a>>> for ColorFormat<'a> {
    fn from(vec: Vec<FormatToken<'a>>) -> Self {
        Self(vec)
    }
}

#[derive(Debug, PartialEq)]
enum ColorParseError<I> {
    InputEmpty,
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for ColorParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        ColorParseError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug, PartialEq)]
enum FormatToken<'a> {
    Color(ColorField),
    Text(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct ColorField {
    symbol: ColorSymbol,
    digit_format: DigitFormat,
}

#[derive(Debug, PartialEq)]
enum DigitFormat {
    Hex,
    UppercaseHex,
    Octal,
    Decimal,
}

impl DigitFormat {
    #[inline]
    pub fn radix(&self) -> u32 {
        match &self {
            DigitFormat::Octal => 8,
            DigitFormat::Decimal => 10,
            DigitFormat::Hex => 16,
            DigitFormat::UppercaseHex => 16,
        }
    }

    #[inline]
    fn format_digit(&self, digit: u32) -> Option<char> {
        let mut ch = std::char::from_digit(digit, self.radix());
        if matches!(self, DigitFormat::UppercaseHex) {
            ch = ch.map(|ch| ch.to_ascii_uppercase());
        }
        ch
    }

    #[inline]
    fn format_num(&self, mut num: u32, text: &mut String, stack: &mut LinkedList<u32>) {
        if num == 0 {
            stack.push_front(num);
        } else {
            while num > 0 {
                stack.push_front(num % self.radix());
                num /= self.radix();
            }
        }

        while let Some(num) = stack.pop_front() {
            if let Some(ch) = self.format_digit(num) {
                text.push(ch);
            }
        }
    }

    fn format_float(mut num: f32, text: &mut String, stack: &mut LinkedList<u32>) {
        if num.is_nan() || num.is_infinite() || num.is_subnormal() {
            return;
        }
        if num.is_sign_negative() {
            text.push('-');
        }

        let radix = DigitFormat::Decimal.radix() as f32;
        num = num.abs();

        DigitFormat::Decimal.format_num(num.trunc() as u32, text, stack);
        text.push('.');

        let mut fract = num.fract() * radix;
        while fract.fract() != 0. {
            fract *= radix;
        }

        DigitFormat::Decimal.format_num(fract as u32, text, stack);
    }
}

impl Default for DigitFormat {
    fn default() -> Self {
        Self::Decimal
    }
}

#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
enum ColorSymbol {
    Red,
    Green,
    Blue,

    Red255,
    Green255,
    Blue255,

    Cyan,
    Magenta,
    Yellow,
    Key,

    Cyan100,
    Magenta100,
    Yellow100,
    Key100,

    HSLHue,
    HSLSaturation,
    HSLLight,

    HSLHue360,
    HSLSaturation100,
    HSLLight100,

    HSVHue,
    HSVSaturation,
    HSVValue,

    HSVHue360,
    HSVSaturation100,
    HSVValue100,

    LabL,
    LabA,
    LabB,
    
    LCHabL,
    LCHabC,
    LCHabH,
    
    LuvL,
    LuvU,
    LuvV,
    
    LCHuvL,
    LCHuvC,
    LCHuvH,
    
    xyYx,
    xyYy,
    xyYY,
    
    XYZx,
    XYZy,
    XYZz,
}

fn parse_rgb_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("r255").map(|_| ColorSymbol::Red255),
        tag("g255").map(|_| ColorSymbol::Green255),
        tag("b255").map(|_| ColorSymbol::Blue255),
        char('r').map(|_| ColorSymbol::Red),
        char('g').map(|_| ColorSymbol::Green),
        char('b').map(|_| ColorSymbol::Blue),
    ))(i)
}

fn parse_cmyk_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("c100").map(|_| ColorSymbol::Cyan100),
        tag("m100").map(|_| ColorSymbol::Magenta100),
        tag("y100").map(|_| ColorSymbol::Yellow100),
        tag("k100").map(|_| ColorSymbol::Key100),
        char('c').map(|_| ColorSymbol::Cyan),
        char('m').map(|_| ColorSymbol::Magenta),
        char('y').map(|_| ColorSymbol::Yellow),
        char('k').map(|_| ColorSymbol::Key),
    ))(i)
}

fn parse_hsl_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("hsl_h360").map(|_| ColorSymbol::HSLHue360),
        tag("hsl_s100").map(|_| ColorSymbol::HSLSaturation100),
        tag("hsl_l100").map(|_| ColorSymbol::HSLLight100),
        tag("hsl_h").map(|_| ColorSymbol::HSLHue),
        tag("hsl_s").map(|_| ColorSymbol::HSLSaturation),
        tag("hsl_l").map(|_| ColorSymbol::HSLLight),
    ))(i)
}

fn parse_hsv_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("hsv_h360").map(|_| ColorSymbol::HSVHue360),
        tag("hsv_s100").map(|_| ColorSymbol::HSVSaturation100),
        tag("hsv_v100").map(|_| ColorSymbol::HSVValue100),
        tag("hsv_h").map(|_| ColorSymbol::HSVHue),
        tag("hsv_s").map(|_| ColorSymbol::HSVSaturation),
        tag("hsv_v").map(|_| ColorSymbol::HSVValue),
    ))(i)
}

fn parse_lab_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("lab_l").map(|_| ColorSymbol::LabL),
        tag("lab_a").map(|_| ColorSymbol::LabA),
        tag("lab_b").map(|_| ColorSymbol::LabB),
    ))(i)
}

fn parse_lch_ab_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("lch_ab_l").map(|_| ColorSymbol::LCHabL),
        tag("lch_ab_c").map(|_| ColorSymbol::LCHabC),
        tag("lch_ab_h").map(|_| ColorSymbol::LCHabH),
    ))(i)
}

fn parse_luv_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("luv_l").map(|_| ColorSymbol::LuvL),
        tag("luv_u").map(|_| ColorSymbol::LuvU),
        tag("luv_v").map(|_| ColorSymbol::LuvV),
    ))(i)
}

fn parse_lch_uv_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("lch_uv_l").map(|_| ColorSymbol::LCHuvL),
        tag("lch_uv_c").map(|_| ColorSymbol::LCHuvC),
        tag("lch_uv_h").map(|_| ColorSymbol::LCHuvH),
    ))(i)
}

fn parse_xyy_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("xyy_x").map(|_| ColorSymbol::xyYx),
        tag("xyy_y").map(|_| ColorSymbol::xyYy),
        tag("xyy_Y").map(|_| ColorSymbol::xyYY),
    ))(i)
}

fn parse_xyz_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("xyz_x").map(|_| ColorSymbol::XYZx),
        tag("xyz_y").map(|_| ColorSymbol::XYZy),
        tag("xyz_z").map(|_| ColorSymbol::XYZz),
    ))(i)
}

fn parse_color_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        parse_rgb_symbol,
        parse_cmyk_symbol,
        parse_hsl_symbol,
        parse_hsv_symbol,
        parse_lab_symbol,
        parse_lch_ab_symbol,
        parse_luv_symbol,
        parse_lch_uv_symbol,
        parse_xyy_symbol,
        parse_xyz_symbol,
    ))(i)
}

fn parse_hex_format(i: &str) -> IResult<&str, DigitFormat, ColorParseError<&str>> {
    map(char('x'), |_| DigitFormat::Hex)(i)
}

fn parse_hex_uppercase_format(i: &str) -> IResult<&str, DigitFormat, ColorParseError<&str>> {
    map(char('X'), |_| DigitFormat::UppercaseHex)(i)
}

fn parse_octal_format(i: &str) -> IResult<&str, DigitFormat, ColorParseError<&str>> {
    map(char('o'), |_| DigitFormat::Octal)(i)
}

fn parse_digit_format(i: &str) -> IResult<&str, DigitFormat, ColorParseError<&str>> {
    preceded(
        char(':'),
        alt((
            parse_hex_format,
            parse_hex_uppercase_format,
            parse_octal_format,
        )),
    )(i)
}

fn parse_color_field(i: &str) -> IResult<&str, ColorField, ColorParseError<&str>> {
    delimited(
        char('{'),
        preceded(
            space0,
            map(
                tuple((parse_color_symbol, opt(parse_digit_format))),
                |(symbol, digit_format)| ColorField {
                    symbol,
                    digit_format: digit_format.unwrap_or_default(),
                },
            ),
        ),
        preceded(space0, char('}')),
    )(i)
}

#[inline]
fn is_not_variable_start(chr: char) -> bool {
    chr != '{'
}

fn parse_text(i: &str) -> IResult<&str, &str, ColorParseError<&str>> {
    if i.is_empty() {
        return Err(Err::Error(ColorParseError::InputEmpty));
    }

    take_while(is_not_variable_start)(i)
}

fn parse_brace(i: &str) -> IResult<&str, FormatToken, ColorParseError<&str>> {
    map(tag("{"), FormatToken::Text)(i)
}

fn parse_format_token(i: &str) -> IResult<&str, FormatToken, ColorParseError<&str>> {
    alt((
        map(parse_color_field, FormatToken::Color),
        parse_brace,
        map(parse_text, FormatToken::Text),
    ))(i)
}

fn parse_color_format(i: &str) -> IResult<&str, ColorFormat, ColorParseError<&str>> {
    map(many0(parse_format_token), ColorFormat)(i)
}

#[cfg(test)]
mod tests {
    use crate::app::color_format::{
        ColorField, ColorFormat, ColorSymbol, DigitFormat, FormatToken,
    };
    use crate::color::{Color, Illuminant, Rgb, RgbWorkingSpace};
    macro_rules! field {
        ($sym:tt) => {
            FormatToken::Color(ColorField {
                symbol: ColorSymbol::$sym,
                digit_format: DigitFormat::default(),
            })
        };
        ($sym:tt, $fmt:tt) => {
            FormatToken::Color(ColorField {
                symbol: ColorSymbol::$sym,
                digit_format: DigitFormat::$fmt,
            })
        };
    }
    macro_rules! test_case {
        ($input:literal, $want:expr) => {
            let parsed = ColorFormat::parse($input).unwrap();
            assert_eq!(parsed, $want);
        };
    }

    #[test]
    fn formats_custom_color_string() {
        macro_rules! test_case {
            ($fmt:literal => $want:literal, $color:expr) => {
                let color_format = ColorFormat::parse($fmt).unwrap();
                let color = $color;
                let formatted =
                    color_format.format_color(&color, RgbWorkingSpace::SRGB, Illuminant::D65);
                assert_eq!(formatted, $want);
            };
        }
        test_case!(
            "{r} {g} {b}" => "0.5 0.5 0.5",
            Color::Rgb(Rgb::new(0.5, 0.5, 0.5))
        );
        test_case!(
            "{r255} {g255} {b255}" => "127 127 127",
            Color::Rgb(Rgb::new(0.5, 0.5, 0.5))
        );
        test_case!(
            "r:0o{r255:o} g:0x{g255:X} b:0x{b255:x}" => "r:0o177 g:0x7F b:0x7f",
            Color::Rgb(Rgb::new(0.5, 0.5, 0.5))
        );
        test_case!(
            "{lab_l} {lab_a} {lab_b}" => "55.6818085 -17.12739562 -27.2706623",
            Color::Rgb(Rgb::new_scaled(35, 144, 180))
        );
    }

    #[test]
    fn parses_basic_color_format() {
        test_case!(
            "{r} {g} {b}",
            vec![
                field!(Red),
                FormatToken::Text(" "),
                field!(Green),
                FormatToken::Text(" "),
                field!(Blue),
            ]
            .into()
        );
        test_case!(
            "{ c }{m} { y  }   {   k }%",
            vec![
                field!(Cyan),
                field!(Magenta),
                FormatToken::Text(" "),
                field!(Yellow),
                FormatToken::Text("   "),
                field!(Key),
                FormatToken::Text("%"),
            ]
            .into()
        );
        test_case!(
            "{hsv_h} {{ {hsv_s} }} {hsv_v}",
            vec![
                field!(HSVHue),
                FormatToken::Text(" "),
                FormatToken::Text("{"),
                FormatToken::Text("{"),
                FormatToken::Text(" "),
                field!(HSVSaturation),
                FormatToken::Text(" }} "),
                field!(HSVValue),
            ]
            .into()
        );
    }

    #[test]
    fn parses_digit_format() {
        test_case!(
            "L:{ lch_ab_l:x } C:{lch_ab_c:X} H:{lch_ab_h:o}",
            vec![
                FormatToken::Text("L:"),
                field!(LCHabL, Hex),
                FormatToken::Text(" C:"),
                field!(LCHabC, UppercaseHex),
                FormatToken::Text(" H:"),
                field!(LCHabH, Octal),
            ]
            .into()
        );
    }
}
