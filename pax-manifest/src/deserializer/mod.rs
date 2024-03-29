use core::panic;
use pax_message::serde::Deserialize;
use pest::Parser;
use serde::de::{self, DeserializeOwned, Visitor};
use serde::forward_to_deserialize_any;

pub mod error;
mod helpers;
mod tests;

use self::helpers::{PaxColor, PaxEnum, PaxObject, PaxSeq};

pub use error::{Error, Result};

use crate::utils::{PaxParser, Rule};

use crate::constants::{
    COLOR, DEGREES, FLOAT, INTEGER, NUMERIC, PERCENT, PIXELS, RADIANS, ROTATION, SIZE, STRING_BOX,
    TRUE,
};

use crate::deserializer::helpers::{ColorFuncArg, PaxSeqArg};
use pax_runtime_api::{Color, IntoableLiteral};

pub struct Deserializer {
    input: String,
}

impl Deserializer {
    pub fn from_string(input: String) -> Self {
        Deserializer { input }
    }
}

// Literal Intoable Graph, as of initial impl:
// Numeric
// - Size
// - Rotation
// - ColorChannel
// Percent
// - ColorChannel
// - Rotation
// - Size
// Color
// - Stroke (1px solid)
// - Fill (solid)

// `from_pax_try_intoable_literal` tries to parse the provided string as a literal type that we know how to coerce into other literal types.
// Enables type-coercion from certain literal values, like `Percent` and `Color`.
// If this string parses into a literal type that can be `Into`d (for example, 10% -> ColorChannel::Percent(10))
// then package the parsed value into the IntoableLiteral enum, which gives us an interface into
// the Rust `Into` system, while appeasing its particular demands around codegen.
pub fn from_pax_try_intoable_literal(str: &str) -> Result<IntoableLiteral> {
    if let Ok(_ast) = PaxParser::parse(Rule::literal_color, str) {
        Ok(IntoableLiteral::Color(from_pax(str).unwrap()))
    } else if let Ok(ast) = PaxParser::parse(Rule::literal_number_with_unit, str) {
        // let mut ast= ast.next().unwrap().into_inner();
        let _number = ast.clone().next().unwrap().as_str();
        let unit = ast.clone().next().unwrap().as_str();
        match unit {
            "%" => Ok(IntoableLiteral::Percent(from_pax(str).unwrap())),
            _ => Err(Error::UnsupportedMethod),
        }
    } else if let Ok(_ast) = PaxParser::parse(Rule::literal_number, str) {
        Ok(IntoableLiteral::Numeric(from_pax(str).unwrap()))
    } else {
        Err(Error::UnsupportedMethod) //Not an IntoableLiteral
    }
}

/// Main entry-point for deserializing a type from Pax.
pub fn from_pax<T>(str: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let deserializer: Deserializer = Deserializer::from_string(str.trim().to_string());
    let t = T::deserialize(deserializer)?;
    Ok(t)
}

impl<'de> de::Deserializer<'de> for Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let ast = if let Ok(ast) = PaxParser::parse(Rule::literal_value, &self.input) {
            ast.clone().next().unwrap()
        } else if let Ok(ast) = PaxParser::parse(Rule::literal_object, &self.input) {
            ast.clone().next().unwrap()
        } else if let Ok(_) = PaxParser::parse(Rule::identifier, &self.input) {
            return self.deserialize_identifier(visitor);
        } else {
            return Err(Error::UnsupportedType(self.input));
        };

        let ret = match ast.as_rule() {
            Rule::literal_value => {
                let inner_pair = ast.clone().into_inner().next().unwrap();
                match inner_pair.as_rule() {
                    Rule::literal_color => {
                        // literal_color = {literal_color_space_func | literal_color_const}
                        let what_kind_of_color = inner_pair.clone().into_inner().next().unwrap();
                        match what_kind_of_color.as_rule() {
                            Rule::literal_color_space_func => {
                                let lcsf_pairs =
                                    inner_pair.clone().into_inner().next().unwrap().into_inner();
                                let func = inner_pair
                                    .clone()
                                    .into_inner()
                                    .next()
                                    .unwrap()
                                    .as_str()
                                    .to_string()
                                    .trim()
                                    .to_string()
                                    .split("(")
                                    .next()
                                    .unwrap()
                                    .to_string();

                                // pre-process each lcsf_pair and wrap into a ColorChannelDefinition

                                //literal_color_channel = {literal_number_with_unit | literal_number_integer}
                                let args = lcsf_pairs
                                    .into_iter()
                                    .map(|lcsf| {
                                        let lcsf = lcsf.into_inner().next().unwrap();
                                        match lcsf.as_rule() {
                                            Rule::literal_number_with_unit => {
                                                let inner = lcsf.clone().into_inner();
                                                let number = inner.clone().next().unwrap().as_str();
                                                let unit = inner.clone().nth(1).unwrap().as_str();
                                                match unit {
                                                    "%" => {
                                                        ColorFuncArg::Percent(number.to_string())
                                                    }
                                                    "rad" | "deg" => ColorFuncArg::Rotation(
                                                        lcsf.as_str().to_string(),
                                                    ),
                                                    _ => {
                                                        unreachable!(); //Unsupported unit
                                                    }
                                                }
                                            }
                                            Rule::literal_number_integer => {
                                                ColorFuncArg::Integer(lcsf.as_str().to_string())
                                            }
                                            _ => {
                                                panic!("{}", lcsf.as_str())
                                            }
                                        }
                                    })
                                    .collect();

                                visitor.visit_enum(PaxColor {
                                    color_func: func,
                                    args,
                                })
                            }
                            Rule::literal_color_const => {
                                let explicit_color = visitor.visit_enum(PaxEnum::new(
                                    Some(COLOR.to_string()),
                                    what_kind_of_color.as_str().to_string(),
                                    None,
                                ));
                                explicit_color
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                    Rule::literal_number => {
                        let number = inner_pair.into_inner().next().unwrap();
                        match number.as_rule() {
                            Rule::literal_number_integer => visitor.visit_enum(PaxEnum::new(
                                Some(NUMERIC.to_string()),
                                INTEGER.to_string(),
                                Some(number.as_str().to_string()),
                            )),
                            Rule::literal_number_float => visitor.visit_enum(PaxEnum::new(
                                Some(NUMERIC.to_string()),
                                FLOAT.to_string(),
                                Some(number.as_str().to_string()),
                            )),
                            _ => Err(Error::UnsupportedType(number.as_str().to_string())),
                        }
                    }
                    Rule::literal_number_with_unit => {
                        let inner = inner_pair.into_inner();
                        let number = inner.clone().next().unwrap().as_str();
                        let unit = inner.clone().nth(1).unwrap().as_str();
                        match unit {
                            "%" => visitor.visit_enum(PaxEnum::new(
                                None,
                                PERCENT.to_string(),
                                Some(number.to_string()),
                            )),
                            "px" => visitor.visit_enum(PaxEnum::new(
                                Some(SIZE.to_string()),
                                PIXELS.to_string(),
                                Some(number.to_string()),
                            )),
                            "rad" => visitor.visit_enum(PaxEnum::new(
                                Some(ROTATION.to_string()),
                                RADIANS.to_string(),
                                Some(number.to_string()),
                            )),
                            "deg" => visitor.visit_enum(PaxEnum::new(
                                Some(ROTATION.to_string()),
                                DEGREES.to_string(),
                                Some(number.to_string()),
                            )),
                            _ => {
                                unreachable!("Unsupported unit: {}", unit)
                            }
                        }
                    }
                    Rule::string => {
                        let string_within_quotes =
                            inner_pair.into_inner().next().unwrap().as_str().to_string();
                        visitor.visit_map(PaxObject::new(
                            Some(STRING_BOX.to_string()),
                            vec![("string".to_string(), string_within_quotes)],
                        ))
                    }
                    Rule::literal_tuple => {
                        let pairs = inner_pair.into_inner();
                        let elements = pairs
                            .map(|pair| PaxSeqArg::String(pair.as_str().to_string()))
                            .collect::<Vec<PaxSeqArg>>();
                        visitor.visit_seq(PaxSeq::new(elements))
                    }

                    Rule::literal_enum_value => {
                        visitor.visit_enum(PaxEnum::from_string(inner_pair.as_str().to_string()))
                    }
                    Rule::literal_boolean => visitor.visit_bool(inner_pair.as_str() == TRUE),
                    _ => Err(Error::UnsupportedType(inner_pair.as_str().to_string())),
                }
            }
            Rule::literal_object => {
                visitor.visit_map(PaxObject::from_string(ast.as_str().to_string()))
            }
            _ => Err(Error::UnsupportedType(ast.as_str().to_string())),
        }?;

        Ok(ret)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum ignored_any
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(&self.input)
    }
}
