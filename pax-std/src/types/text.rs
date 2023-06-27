use std::path::PathBuf;
use pax_lang::api::{Interpolatable, PropertyInstance, PropertyLiteral, Size2D, SizePixels};
use pax_message::{FontPatch, FontWeightMessage, FontStyleMessage, LocalFontMessage, SystemFontMessage, TextAlignHorizontalMessage, TextAlignVerticalMessage, WebFontMessage, LinkStyleMessage};
use pax_lang::*;
use pax_lang::api::numeric::Numeric;
use crate::types::Color;

#[derive(Pax)]
#[custom(Default)]
pub enum Font {
    System(SystemFont),
    Web(WebFont),
    Local(LocalFont),
}

impl Default for Font {
    fn default() -> Self {
        Self::System(SystemFont::default())
    }
}

#[derive(Pax)]
#[custom(Imports, Default)]
pub struct SystemFont {
    pub family: String,
    pub style: FontStyle,
    pub weight: FontWeight,
}

impl Default for SystemFont {
    fn default() -> Self {
        Self {
            family: "Arial".to_string(),
            style: FontStyle::Italic,
            weight: FontWeight::Bold,
        }
    }
}


#[derive(Pax)]
#[custom(Imports)]
pub struct WebFont {
    pub family: String,
    pub url: String,
    pub style: FontStyle,
    pub weight: FontWeight,
}

#[derive(Pax)]
#[custom(Imports)]
pub struct LocalFont {
    pub family: String,
    pub path: String,
    pub style: FontStyle,
    pub weight: FontWeight,
}


#[derive(Pax)]
#[custom(Imports)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}


#[derive(Pax)]
#[custom(Imports)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}


#[derive(Pax)]
#[custom(Imports)]
pub enum TextAlignHorizontal {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Pax)]
#[custom(Imports)]
pub enum TextAlignVertical {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Pax)]
#[custom(Imports)]
pub struct LinkStyle {
    pub font: Option<Font>,
    pub fill: Color,
    pub underline: bool,
    pub size: SizePixels,
}

#[derive(Pax)]
#[custom(Imports)]
pub struct SizeWrapper {
    pub size: SizePixels,
}

impl SizeWrapper {
    pub fn set(x: Numeric) -> Self{
        Self {
            size: SizePixels(x)
        }
    }
}

impl Into<TextAlignHorizontalMessage> for &TextAlignHorizontal {
    fn into(self) -> TextAlignHorizontalMessage {
        match self {
            TextAlignHorizontal::Center => {TextAlignHorizontalMessage::Center}
            TextAlignHorizontal::Left => {TextAlignHorizontalMessage::Left}
            TextAlignHorizontal::Right => {TextAlignHorizontalMessage::Right}
        }
    }
}

impl PartialEq<TextAlignHorizontalMessage> for TextAlignHorizontal {
    fn eq(&self, other: &TextAlignHorizontalMessage) -> bool {
        match (self, other) {
            (TextAlignHorizontal::Center, TextAlignHorizontalMessage::Center) => true,
            (TextAlignHorizontal::Left, TextAlignHorizontalMessage::Left) => true,
            (TextAlignHorizontal::Right, TextAlignHorizontalMessage::Right) => true,
            _ => false,
        }
    }
}

pub fn opt_align_to_message(opt_alignment: &Option<TextAlignHorizontal>) -> Option<TextAlignHorizontalMessage> {
    opt_alignment.as_ref().map(|alignment| {
        match alignment {
            TextAlignHorizontal::Center => TextAlignHorizontalMessage::Center,
            TextAlignHorizontal::Left => TextAlignHorizontalMessage::Left,
            TextAlignHorizontal::Right => TextAlignHorizontalMessage::Right,
        }
    })
}

pub fn opt_link_style_to_message(opt_link_style: &Option<LinkStyle>) -> Option<LinkStyleMessage> {
    opt_link_style.as_ref().map(|link_style| link_style.clone().into())
}

pub fn opt_value_eq_opt_msg<T, U>(opt_value: &Option<T>, opt_value_msg: &Option<U>) -> bool
    where
        T: PartialEq<U>,
{
    match (opt_value, opt_value_msg) {
        (Some(value), Some(value_msg)) => value.eq(value_msg),
        (None, None) => true,
        _ => false,
    }
}

impl PartialEq<f64> for SizeWrapper {
    fn eq(&self, other: &f64) -> bool {
        self.size == Numeric::from(other.clone())
    }
}

impl Into<TextAlignVerticalMessage> for &TextAlignVertical {
    fn into(self) -> TextAlignVerticalMessage {
        match self {
            TextAlignVertical::Top => TextAlignVerticalMessage::Top,
            TextAlignVertical::Center => TextAlignVerticalMessage::Center,
            TextAlignVertical::Bottom => TextAlignVerticalMessage::Bottom,
        }
    }
}

impl PartialEq<TextAlignVerticalMessage> for TextAlignVertical {
    fn eq(&self, other: &TextAlignVerticalMessage) -> bool {
        match (self, other) {
            (TextAlignVertical::Top, TextAlignVerticalMessage::Top) => true,
            (TextAlignVertical::Center, TextAlignVerticalMessage::Center) => true,
            (TextAlignVertical::Bottom, TextAlignVerticalMessage::Bottom) => true,
            _ => false,
        }
    }
}

impl From<Font> for FontPatch {
    fn from(font: Font) -> Self {
        match font {
            Font::System(system_font) => FontPatch::System(SystemFontMessage {
                family: Some(system_font.family),
                style: Some(system_font.style.into()),
                weight: Some(system_font.weight.into()),
            }),
            Font::Web(web_font) => FontPatch::Web(WebFontMessage {
                family: Some(web_font.family),
                url: Some(web_font.url),
                style: Some(web_font.style.into()),
                weight: Some(web_font.weight.into()),
            }),
            Font::Local(local_font) => FontPatch::Local(LocalFontMessage {
                family: Some(local_font.family),
                path: Some(local_font.path),
                style: Some(local_font.style.into()),
                weight: Some(local_font.weight.into()),
            }),
        }
    }
}

impl PartialEq<FontStyleMessage> for FontStyle {
    fn eq(&self, other: &FontStyleMessage) -> bool {
        match (self, other) {
            (FontStyle::Normal, FontStyleMessage::Normal) => true,
            (FontStyle::Italic, FontStyleMessage::Italic) => true,
            (FontStyle::Oblique, FontStyleMessage::Oblique) => true,
            _ => false,
        }
    }
}

impl PartialEq<FontWeightMessage> for FontWeight {
    fn eq(&self, other: &FontWeightMessage) -> bool {
        match (self, other) {
            (FontWeight::Thin, FontWeightMessage::Thin) => true,
            (FontWeight::ExtraLight, FontWeightMessage::ExtraLight) => true,
            (FontWeight::Light, FontWeightMessage::Light) => true,
            (FontWeight::Normal, FontWeightMessage::Normal) => true,
            (FontWeight::Medium, FontWeightMessage::Medium) => true,
            (FontWeight::SemiBold, FontWeightMessage::SemiBold) => true,
            (FontWeight::Bold, FontWeightMessage::Bold) => true,
            (FontWeight::ExtraBold, FontWeightMessage::ExtraBold) => true,
            (FontWeight::Black, FontWeightMessage::Black) => true,
            _ => false,
        }
    }
}

impl PartialEq<FontPatch> for Font {
    fn eq(&self, other: &FontPatch) -> bool {
        match (self, other) {
            (Font::System(system_font), FontPatch::System(system_font_patch)) => {
                system_font_patch.family.as_ref().map_or(false, |family| *family == system_font.family)
                    && system_font_patch.style.as_ref().map_or(false, |style| system_font.style.eq(style))
                    && system_font_patch.weight.as_ref().map_or(false, |weight| system_font.weight.eq(weight))
            }
            (Font::Web(web_font), FontPatch::Web(web_font_patch)) => {
                web_font_patch.family.as_ref().map_or(false, |family| *family == web_font.family)
                    && web_font_patch.url.as_ref().map_or(false, |url| *url == web_font.url)
                    && web_font_patch.style.as_ref().map_or(false, |style| web_font.style.eq(style))
                    && web_font_patch.weight.as_ref().map_or(false, |weight| web_font.weight.eq(weight))
            }
            (Font::Local(local_font), FontPatch::Local(local_font_patch)) => {
                local_font_patch.family.as_ref().map_or(false, |family| *family == local_font.family)
                    && local_font_patch.path.as_ref().map_or(false, |path| *path == local_font.path)
                    && local_font_patch.style.as_ref().map_or(false, |style| local_font.style.eq(style))
                    && local_font_patch.weight.as_ref().map_or(false, |weight| local_font.weight.eq(weight))
            }
            _ => false,
        }
    }
}

impl Font {
    pub fn system(family: String, style: FontStyle, weight: FontWeight) -> Self {
        Self::System(SystemFont { family, style, weight })
    }

    pub fn web(
        family: String,
        url: String,
        style: FontStyle,
        weight: FontWeight,
    ) -> Self {
        Self::Web(WebFont {
            family,
            url,
            style,
            weight,
        })
    }

    pub fn local(family: String, path: String, style: FontStyle, weight: FontWeight) -> Self {
        Self::Local(LocalFont { family, path, style, weight })
    }
}

impl From<FontStyleMessage> for FontStyle {
    fn from(style_msg: FontStyleMessage) -> Self {
        match style_msg {
            FontStyleMessage::Normal => FontStyle::Normal,
            FontStyleMessage::Italic => FontStyle::Italic,
            FontStyleMessage::Oblique => FontStyle::Oblique,
        }
    }
}

impl From<FontStyle> for FontStyleMessage {
    fn from(style: FontStyle) -> Self {
        match style {
            FontStyle::Normal => FontStyleMessage::Normal,
            FontStyle::Italic => FontStyleMessage::Italic,
            FontStyle::Oblique => FontStyleMessage::Oblique,
        }
    }
}


impl From<FontWeightMessage> for FontWeight {
    fn from(weight_msg: FontWeightMessage) -> Self {
        match weight_msg {
            FontWeightMessage::Thin => FontWeight::Thin,
            FontWeightMessage::ExtraLight => FontWeight::ExtraLight,
            FontWeightMessage::Light => FontWeight::Light,
            FontWeightMessage::Normal => FontWeight::Normal,
            FontWeightMessage::Medium => FontWeight::Medium,
            FontWeightMessage::SemiBold => FontWeight::SemiBold,
            FontWeightMessage::Bold => FontWeight::Bold,
            FontWeightMessage::ExtraBold => FontWeight::ExtraBold,
            FontWeightMessage::Black => FontWeight::Black,
        }
    }
}

impl From<FontWeight> for FontWeightMessage {
    fn from(weight: FontWeight) -> Self {
        match weight {
            FontWeight::Thin => FontWeightMessage::Thin,
            FontWeight::ExtraLight => FontWeightMessage::ExtraLight,
            FontWeight::Light => FontWeightMessage::Light,
            FontWeight::Normal => FontWeightMessage::Normal,
            FontWeight::Medium => FontWeightMessage::Medium,
            FontWeight::SemiBold => FontWeightMessage::SemiBold,
            FontWeight::Bold => FontWeightMessage::Bold,
            FontWeight::ExtraBold => FontWeightMessage::ExtraBold,
            FontWeight::Black => FontWeightMessage::Black,
        }
    }
}

impl PartialEq<LinkStyleMessage> for LinkStyle {
    fn eq(&self, other: &LinkStyleMessage) -> bool {
        other.font.as_ref().map_or(false, |font| self.font.as_ref().map_or(false, |s_font| s_font.eq(font)))
            && other.fill.as_ref().map_or(false, |fill| self.fill.eq(fill))
            && other.underline.as_ref().map_or(false, |underline| *underline == self.underline)
            && other.size.as_ref().map_or(false, |size| self.size.eq(&Numeric::Float(*size)))
    }
}

impl From<LinkStyle> for LinkStyleMessage {
    fn from(ls: LinkStyle) -> Self {
        LinkStyleMessage {
            font: ls.font.map(|f| f.into()),
            fill: Some((&ls.fill).into()),
            underline: Some(ls.underline),
            size: Some(ls.size.0.get_as_float()),
        }
    }
}

impl LinkStyle {
    pub fn new(font: Option<Font>, fill: Color, underline: bool, size: SizePixels) -> Self {
        Self { font, fill, underline, size }
    }

    pub fn arbitrary() -> Option<Self> {
        Some(Self {
            font: Some(Font::system("Arial".to_string(), FontStyle::Normal, FontWeight::Bold)),
            fill: Color::rgba(0.0.into(), 0.0.into(), 1.0.into(), 1.0.into()),
            underline: true,
            size: SizePixels(Numeric::Float(48.0)),
        })
    }
}