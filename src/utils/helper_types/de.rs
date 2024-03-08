use std::fmt::Formatter;
use std::marker::PhantomData;
use std::str::FromStr;
use serde::{Deserialize, Deserializer};
use serde::de::Visitor;

pub fn str_or_struct_deserializer<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
  T: Deserialize<'de> + FromStr<Err = anyhow::Error>,
  D: Deserializer<'de>,
{
  struct StringOrStruct<T>(PhantomData<fn() -> T>);

  impl<'de, T> Visitor<'de> for StringOrStruct<T>
  where
    T: Deserialize<'de> + FromStr<Err = anyhow::Error>,
  {
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
      formatter.write_str("string or map")
    }

    fn visit_str<E>(self, value: &str) -> Result<T, E>
    where
      E: serde::de::Error,
    {
      Ok(FromStr::from_str(value).unwrap())
    }

    fn visit_map<M>(self, map: M) -> Result<T, M::Error>
    where
      M: serde::de::MapAccess<'de>,
    {
      Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
    }
  }

  deserializer.deserialize_any(StringOrStruct(PhantomData))
}