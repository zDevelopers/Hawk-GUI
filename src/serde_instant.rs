use std::str::FromStr;
use core::fmt;

use datetime;
use datetime::ISO;


#[derive(Debug, Clone)]
pub struct Instant {
    pub instant: datetime::Instant
}

impl serde::Serialize for Instant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(datetime::LocalDateTime::from_instant(self.instant).iso().to_string().as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Instant {
    fn deserialize<D>(deserializer: D) -> Result<Instant, D::Error> where D: serde::Deserializer<'de> {
        struct InstantVisitor;

        impl<'vi> serde::de::Visitor<'vi> for InstantVisitor {
            type Value = Instant;

            fn expecting(
                &self,
                formatter: &mut fmt::Formatter,
            ) -> fmt::Result {
                write!(formatter, "a date with ISO 8601 format")
            }

            fn visit_str<E: serde::de::Error>(
                self,
                value: &str,
            ) -> Result<Instant, E> {
                match datetime::LocalDateTime::from_str(value) {
                    Ok(local_date_time) => Ok(Instant { instant: local_date_time.to_instant() }),
                    Err(e) => Err(e).map_err(E::custom)
                }
            }
        }

        deserializer.deserialize_str(InstantVisitor)
    }
}
