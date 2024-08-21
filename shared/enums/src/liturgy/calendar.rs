use core::fmt;

use serde::{
    de::{Deserializer, Error as DeserializeError, Visitor},
    Deserialize, Serialize, Serializer,
};
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display)]
pub enum LiturgicalCalendar {
    #[default]
    #[serde(rename = "roman")]
    #[strum(serialize = "roman")]
    GeneralRoman,
    #[serde(rename = "br")]
    #[strum(serialize = "br")]
    Brazil,
    #[serde(rename = "us")]
    #[strum(serialize = "us")]
    UnitedStatesOfAmerica,
}

// impl Serialize for LiturgicalCalendar {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let value = match *self {
//             Self::GeneralRoman => String::from("roman"),
//             Self::Brazil => String::from("br"),
//             Self::UnitedStatesOfAmerica => String::from("usa"),
//         };

//         serializer.serialize_str(&value)
//     }
// }

// impl<'de> Deserialize<'de> for LiturgicalCalendar {
//     fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
//         struct LiturgicalCalendarVisitor;

//         impl<'de> Visitor<'de> for LiturgicalCalendarVisitor {
//             type Value = LiturgicalCalendar;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("a str represeting LiturgicalCalendar variant")
//             }

//             fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//             where
//                 E: DeserializeError,
//             {
//                 match v {
//                     "br" => Ok(Self::Value::Brazil),
//                     "usa" => Ok(Self::Value::UnitedStatesOfAmerica),
//                     _ => Ok(Self::Value::GeneralRoman),
//                 }
//             }

//             fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
//             where
//                 E: DeserializeError,
//             {
//                 self.visit_str(&v)
//             }
//         }

//         deserializer.deserialize_str(LiturgicalCalendarVisitor)
//     }
// }
