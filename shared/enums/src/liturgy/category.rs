use chrono::{DateTime, Datelike, TimeZone, Weekday};
use core::fmt;
use std::default;
use serde::de::{Deserializer, Error as DeserializeError, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::cmp::{Ord, Ordering};

use super::LiturgicalTime;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CommemorationCategory {
    /// 1. Paschal Triduum, Ressurection of Our Lord and Savor Jesus Christ
    /// 2. The Nativity of The Lord, Epiphany of The Lord, Ascension of The Lord, Pentecost
    /// Sundays on Advent, Lent, Easter and Ash Wednesday,
    /// Holy Week days, from Monday to Thursday, included
    /// Easter Week days, from Monday to Saturday, included
    /// 3. Lord's Solemnities, Blessed Virigin Mary's Solemnities, General Calendar Saints' solemnities
    /// All Souls Day
    /// 4. Proper solemnities, to be:
    /// a) Place or City Patron Saint
    /// b) Church's Dedication Solemnity
    /// c) Church's Bearer Solemnity
    /// d) Main bearer/founder/patron's of order/congregation Solemnity
    Solemnity(u8),
    /// 5. Lord's Feasts on General Calendar
    /// 6. Sundays on Christmas or Ordinary Time
    /// 7. Blessed Virgin Mary's and General Calendar Saints' Feasts
    /// 8. Proper Feasts, to be:
    /// a) Diocesis main Patron Feast
    /// b) Birthday of Dedication of Cathedral's Feast
    /// c) Region, Province, Country or Territory Patron's Feast
    /// d) Main bearer/founder/patron's of order/congregation Feasts, except provided on 4d
    /// e) Other church's proper Feasts
    /// f) Diocesis/Order/Congregation's other proper Feasts
    /// 9. Advent Week Days, from Dec-17th to Dec-24th, included
    /// Days on Christmas Octave
    /// Lent week days
    Feast(u8),
    /// 10. General Calendary's obligatory memorials
    /// 11. Proper obligatory Memorials, to be:
    /// a) Place/Diocesis/Nation/Territory/Order/Congregation's secondary Patron
    /// b) Church's other proper obligatory memorials
    /// c) Church's other proper obligatory memorials written in Diocesis/Order/Congregation's Calendar
    Memorial(u8),
    /// 12. Optional Memorials that might be celebrated at days mentioned in 9. according to descriptions on Mass and Office
    OptionalMemorial(bool),
    /// 13. Advent Week days, until Dec-16h, included
    /// Christmas Time week days, from Jan 2nd until Saturday after Epiphany
    /// Paschal Time Week days, from Monday after Paschal Octave, until Saturday before Pentecost, included
    /// Week days on Ordinary Time
    #[default]
    Regular,
}

impl Serialize for CommemorationCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match *self {
            Self::Solemnity(degree) => format!("S_{}", degree),
            Self::Feast(degree) => format!("F_{}", degree),
            Self::Memorial(degree) => format!("M_{}", degree),
            Self::OptionalMemorial(is_optional_commemoration) => {
                if is_optional_commemoration {
                    String::from("m*")
                } else {
                    String::from("m")
                }
            }
            Self::Regular => String::from(""),
        };

        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for CommemorationCategory {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CommemorationCategoryVisitor;

        impl<'de> Visitor<'de> for CommemorationCategoryVisitor {
            type Value = CommemorationCategory;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a str represeting CommemorationCategory variant")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: DeserializeError,
            {
                match v {
                    s if s.starts_with("S_") || s.starts_with("F_") || s.starts_with("M_") => {
                        let parts: Vec<String> = s.split('_').map(String::from).collect();
                        let commemoration = parts
                            .get(0)
                            .expect("expect commemoration type to be defined");

                        let degree = parts
                            .get(1)
                            .expect("degree str to be defined")
                            .parse::<u8>()
                            .expect("degree str to be parseable as u8");

                        if commemoration == "S" {
                            Ok(Self::Value::Solemnity(degree))
                        } else if commemoration == "F" {
                            Ok(Self::Value::Feast(degree))
                        } else if commemoration == "M" {
                            Ok(Self::Value::Memorial(degree))
                        } else {
                            Err(DeserializeError::custom(format!(
                                "{} doesn't match Commemoration match schema",
                                s
                            )))
                        }
                    }

                    "m" => Ok(Self::Value::OptionalMemorial(false)),
                    "m*" => Ok(Self::Value::OptionalMemorial(true)),
                    "" => Ok(Self::Value::Regular),
                    s => Err(DeserializeError::custom(format!(
                        "{} couldn't be parsed into CommemorationCategory",
                        s
                    ))),
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: DeserializeError,
            {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_str(CommemorationCategoryVisitor)
    }
}

impl CommemorationCategory {
    pub fn has_i_vespers<Tz: TimeZone>(
        &self,
        date: DateTime<Tz>,
        liturgical_time: LiturgicalTime,
    ) -> bool {
        match &self {
            // Também as solenidades, como os domingos, começam no dia precedente, com as I Vésperas
            Self::Solemnity(_) => true,
            // Já as “festas do Senhor”, quando caem em domingo do Tempo Comum, ocupam o lugar deste, com três leituras e com I Vésperas
            // Note that the Commemoration "The Holy Family of Jesus, Mary and Joseph" must be set asa Lord Feast in order for it to comply
            // with a “festa” da Sagrada Família, esta no domingo que venha a cair entre os dias 26 e 31 de dezembro, nesse caso como “solenidade” e, portanto, com I Vésperas
            Self::Feast(degree) => {
                if (*degree == 5 || *degree == 6)
                    && liturgical_time == LiturgicalTime::Ordinary
                    && date.weekday() == Weekday::Sun
                {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl PartialOrd for CommemorationCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Solemnity(sd), Self::Solemnity(od)) => {
                if sd > od {
                    Some(Ordering::Less)
                } else if sd < od {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Self::Feast(sd), Self::Feast(od)) => {
                if sd > od {
                    Some(Ordering::Less)
                } else if sd < od {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Self::Memorial(sd), Self::Memorial(od)) => {
                if sd > od {
                    Some(Ordering::Less)
                } else if sd < od {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Self::OptionalMemorial(s_is_opt_comm), Self::OptionalMemorial(o_is_opt_comm)) => {
                if (*s_is_opt_comm && *o_is_opt_comm) || (!s_is_opt_comm && !o_is_opt_comm) {
                    Some(Ordering::Equal)
                } else if *s_is_opt_comm && !*o_is_opt_comm {
                    Some(Ordering::Greater)
                } else if !*s_is_opt_comm && *o_is_opt_comm {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Self::Regular, Self::Regular) => Some(Ordering::Equal),
            (Self::Solemnity(_), _) => Some(Ordering::Greater),
            (Self::Feast(_), Self::Solemnity(_)) => Some(Ordering::Less),
            (Self::Feast(_), _) => Some(Ordering::Greater),
            (Self::Memorial(_), Self::Solemnity(_)) => Some(Ordering::Less),
            (Self::Memorial(_), Self::Feast(_)) => Some(Ordering::Less),
            (Self::Memorial(_), _) => Some(Ordering::Greater),
            (Self::OptionalMemorial(_), Self::Regular) => Some(Ordering::Greater),
            (Self::OptionalMemorial(_), _) => Some(Ordering::Less),
            (Self::Regular, _) => Some(Ordering::Less),
        }
    }
}

impl Ord for CommemorationCategory {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Solemnity(sd), Self::Solemnity(od)) => {
                if sd > od {
                    Ordering::Less
                } else if sd < od {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (Self::Feast(sd), Self::Feast(od)) => {
                if sd > od {
                    Ordering::Less
                } else if sd < od {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (Self::Memorial(sd), Self::Memorial(od)) => {
                if sd > od {
                    Ordering::Less
                } else if sd < od {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (Self::OptionalMemorial(s_is_opt_comm), Self::OptionalMemorial(o_is_opt_comm)) => {
                if (*s_is_opt_comm && *o_is_opt_comm) || (!s_is_opt_comm && !o_is_opt_comm) {
                    Ordering::Equal
                } else if *s_is_opt_comm && !*o_is_opt_comm {
                    Ordering::Greater
                } else if !*s_is_opt_comm && *o_is_opt_comm {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
            (Self::Regular, Self::Regular) => Ordering::Equal,
            (Self::Solemnity(_), _) => Ordering::Greater,
            (Self::Feast(_), Self::Solemnity(_)) => Ordering::Less,
            (Self::Feast(_), _) => Ordering::Greater,
            (Self::Memorial(_), Self::Solemnity(_)) => Ordering::Less,
            (Self::Memorial(_), Self::Feast(_)) => Ordering::Less,
            (Self::Memorial(_), _) => Ordering::Greater,
            (Self::OptionalMemorial(_), Self::Regular) => Ordering::Greater,
            (Self::OptionalMemorial(_), _) => Ordering::Less,
            (Self::Regular, _) => Ordering::Less,
        }
    }
}
