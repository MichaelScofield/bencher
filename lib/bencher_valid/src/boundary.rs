use derive_more::Display;
use ordered_float::OrderedFloat;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use std::fmt;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::ValidError;

#[typeshare::typeshare]
#[derive(Debug, Display, Clone, Eq, PartialEq, Hash, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Boundary(OrderedFloat<f64>);

impl<'de> Deserialize<'de> for Boundary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_f64(BoundaryVisitor)
    }
}

struct BoundaryVisitor;

impl<'de> Visitor<'de> for BoundaryVisitor {
    type Value = Boundary;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a statistical boundary between [0.5, 1.0)")
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        is_valid_boundary(value)
            .then(|| Boundary(value.into()))
            .ok_or(E::custom(ValidError::InvalidBoundary(value)))
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn is_valid_boundary(boundary: f64) -> bool {
    // The boundary must be greater than or equal to 0.5 and less than 1.0
    if boundary < 0.5 {
        false
    } else {
        boundary < 1.0
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::{is_valid_boundary, Boundary};

    #[test]
    #[allow(clippy::excessive_precision)]
    fn test_boundary() {
        assert_eq!(true, is_valid_boundary(0.49999999999999999));
        assert_eq!(true, is_valid_boundary(0.5));
        assert_eq!(true, is_valid_boundary(0.6));
        assert_eq!(true, is_valid_boundary(0.7));
        assert_eq!(true, is_valid_boundary(0.8));
        assert_eq!(true, is_valid_boundary(0.9));
        assert_eq!(true, is_valid_boundary(0.9999999999999999));

        assert_eq!(false, is_valid_boundary(-1.0));
        assert_eq!(false, is_valid_boundary(0.0));
        assert_eq!(false, is_valid_boundary(0.1));
        assert_eq!(false, is_valid_boundary(0.2));
        assert_eq!(false, is_valid_boundary(0.3));
        assert_eq!(false, is_valid_boundary(0.4));
        assert_eq!(false, is_valid_boundary(0.4999999999999999));
        assert_eq!(false, is_valid_boundary(0.99999999999999999));
        assert_eq!(false, is_valid_boundary(1.0));
        assert_eq!(false, is_valid_boundary(2.0));
        assert_eq!(false, is_valid_boundary(3.0));
    }

    #[test]
    fn test_boundary_serde() {
        let boundary: Boundary = serde_json::from_str("0.49999999999999999").unwrap();
        assert_eq!(Boundary(0.5.into()), boundary);
        let boundary: Boundary = serde_json::from_str("0.5").unwrap();
        assert_eq!(Boundary(0.5.into()), boundary);
        let boundary: Boundary = serde_json::from_str("0.6").unwrap();
        assert_eq!(Boundary(0.6.into()), boundary);
        let boundary: Boundary = serde_json::from_str("0.7").unwrap();
        assert_eq!(Boundary(0.7.into()), boundary);
        let boundary: Boundary = serde_json::from_str("0.8").unwrap();
        assert_eq!(Boundary(0.8.into()), boundary);
        let boundary: Boundary = serde_json::from_str("0.9").unwrap();
        assert_eq!(Boundary(0.9.into()), boundary);
        let boundary: Boundary = serde_json::from_str("0.999999999999999").unwrap();
        assert_eq!(Boundary(0.999999999999999.into()), boundary);

        let boundary = serde_json::from_str::<Boundary>("-1.0");
        assert!(boundary.is_err());
        let boundary = serde_json::from_str::<Boundary>("0.0");
        assert!(boundary.is_err());
        let boundary = serde_json::from_str::<Boundary>("0.1");
        assert!(boundary.is_err());
        let boundary = serde_json::from_str::<Boundary>("0.2");
        assert!(boundary.is_err());
        let boundary = serde_json::from_str::<Boundary>("0.3");
        assert!(boundary.is_err());
        let boundary = serde_json::from_str::<Boundary>("0.4");
        assert!(boundary.is_err());
        let boundary = serde_json::from_str::<Boundary>("0.4999999999999999");
        assert!(boundary.is_err());
        let boundary = serde_json::from_str::<Boundary>("0.99999999999999999");
        assert!(boundary.is_err());
        let boundary = serde_json::from_str::<Boundary>("1.0");
        assert!(boundary.is_err());
        let boundary = serde_json::from_str::<Boundary>("2.0");
        assert!(boundary.is_err());
        let boundary = serde_json::from_str::<Boundary>("3.0");
        assert!(boundary.is_err());
    }
}
