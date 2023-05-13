use std::path::PathBuf;
use pax::api::{Interpolatable, PropertyLiteral};
use pax_message::{FontPatch, FontWeightMessage, WebFontFormatMessage, FontStyleMessage,  LocalFontMessage, SystemFontMessage, TextAlignHorizontalMessage, TextAlignVerticalMessage, WebFontMessage};
use pax_message::reflection::PathQualifiable;
use pax::*;

#[derive(Clone)]
#[pax_type]
pub enum Font {
    System(SystemFont),
    Web(WebFont),
    Local(LocalFont),
}


#[derive(Clone)]
#[pax_type]
pub struct SystemFont {
    pub family: String,
    pub style: FontStyle,
    pub weight: FontWeight,
}


#[derive(Clone)]
#[pax_type]
pub struct WebFont {
    pub family: String,
    pub url: String,
    pub format: WebFontFormat,
    pub style: FontStyle,
    pub weight: FontWeight,
}


#[derive(Clone)]
#[pax_type]
pub enum WebFontFormat {
    TTF,
    OTF,
}


#[derive(Clone)]
#[pax_type]
pub struct LocalFont {
    pub family: String,
    pub path: String,
    pub style: FontStyle,
    pub weight: FontWeight,
}


#[derive(Clone, Default)]
#[pax_type]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}


#[derive(Clone, Default)]
#[pax_type]
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

impl Default for SystemFont {
    fn default() -> Self {
        Self {
            family: "Arial".to_string(),
            style: FontStyle::Italic,
            weight: FontWeight::Bold,
        }
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::System(SystemFont::default())
    }
}

#[derive(Clone, Default)]
#[pax_type]
pub enum TextAlignHorizontal {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Clone, Default)]
#[pax_type]
pub enum TextAlignVertical {
    #[default]
    Top,
    Center,
    Bottom,
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

pub fn opt_alignment_to_message(opt_alignment: &Option<TextAlignHorizontal>) -> Option<TextAlignHorizontalMessage> {
    opt_alignment.as_ref().map(|alignment| {
        match alignment {
            TextAlignHorizontal::Center => TextAlignHorizontalMessage::Center,
            TextAlignHorizontal::Left => TextAlignHorizontalMessage::Left,
            TextAlignHorizontal::Right => TextAlignHorizontalMessage::Right,
        }
    })
}

pub fn opt_alignment_eq_opt_msg(opt_alignment: &Option<TextAlignHorizontal>, opt_alignment_msg: &Option<TextAlignHorizontalMessage>) -> bool {
    match (opt_alignment, opt_alignment_msg) {
        (Some(alignment), Some(alignment_msg)) => alignment.eq(alignment_msg),
        (None, None) => true,
        _ => false,
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
                format: Some(web_font.format.into()),
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

impl PartialEq<WebFontFormatMessage> for WebFontFormat {
    fn eq(&self, other: &WebFontFormatMessage) -> bool {
        match (self, other) {
            (WebFontFormat::TTF, WebFontFormatMessage::TTF) => true,
            (WebFontFormat::OTF, WebFontFormatMessage::OTF) => true,
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
                    && web_font_patch.format.as_ref().map_or(false, |format| web_font.format.eq(format))
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



impl Interpolatable for Font {}

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
            format: WebFontFormat::OTF,
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

impl From<WebFontFormatMessage> for WebFontFormat {
    fn from(format_msg: WebFontFormatMessage) -> Self {
        match format_msg {
            WebFontFormatMessage::TTF => WebFontFormat::TTF,
            WebFontFormatMessage::OTF => WebFontFormat::OTF,
        }
    }
}

impl From<WebFontFormat> for WebFontFormatMessage {
    fn from(format: WebFontFormat) -> Self {
        match format {
            WebFontFormat::TTF => WebFontFormatMessage::TTF,
            WebFontFormat::OTF => WebFontFormatMessage::OTF,
        }
    }
}


// impl PathQualifiable for Font {
//     fn get_fully_qualified_path(atomic_self_type: &str) -> String {
//         "pax::api::Size".to_string()
//     }
// }