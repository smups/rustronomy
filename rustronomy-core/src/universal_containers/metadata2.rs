use std::string::ParseError;

pub trait MetaDataTag: Sized {
  const KEY: &'static str;
  type ValueType;

  fn new(val: Self::ValueType) -> Self;
  fn into(self) -> Self::ValueType;
  fn parse_str(text: &str) -> Result<Self, ParseError>;
  fn to_string(self) -> String;
}

pub trait MetaDataContainer {

}