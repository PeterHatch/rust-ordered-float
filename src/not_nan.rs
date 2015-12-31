extern crate num;
extern crate unreachable;

use std::cmp::Ordering;
use std::error::Error;
use std::ops::Deref;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::io;
use unreachable::unreachable;
use num::Float;

/// A wrapper around Floats providing an implementation of Ord.
#[derive(PartialOrd, PartialEq, Debug, Default, Clone, Copy)]
pub struct NotNaN<T: Float>(T);

impl<T: Float> NotNaN<T> {
    /// Create a NotNaN value.
    ///
    /// Returns Err if val is NaN
    pub fn new(val: T) -> Result<Self, FloatIsNaN> {
        match val {
            ref val if val.is_nan() => Err(FloatIsNaN),
            val => Ok(NotNaN(val)),
        }
    }

    /// Create a NotNaN value from a value that is guaranteed to not be NaN
    ///
    /// Behaviour is undefined if `val` is NaN
    pub unsafe fn unchecked_new(val: T) -> Self {
        debug_assert!(!val.is_nan());
        NotNaN(val)
    }

    /// Get the value out.
    pub fn into_inner(self) -> T {
        let NotNaN(val) = self;
        val
    }
}

impl<T: Float> AsRef<T> for NotNaN<T> {
    fn as_ref(&self) -> &T {
        let NotNaN(ref val) = *self;
        val
    }
}

impl<T: Float + PartialOrd> Ord for NotNaN<T> {
    fn cmp(&self, other: &NotNaN<T>) -> Ordering {
        match self.partial_cmp(&other) {
            Some(ord) => ord,
            None => unsafe { unreachable() },
        }
    }
}

impl<T: Float> Hash for NotNaN<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (man, exp, sign) = self.as_ref().integer_decode();
        (man ^ exp as u64 ^ sign as u64).hash(state)
    }
}

impl<T: Float + fmt::Display> fmt::Display for NotNaN<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl Into<f32> for NotNaN<f32> {
    fn into(self) -> f32 {
        self.into_inner()
    }
}

impl Into<f64> for NotNaN<f64> {
    fn into(self) -> f64 {
        self.into_inner()
    }
}

impl<T: Float> From<T> for NotNaN<T> {
    fn from(v: T) -> Self {
        assert!(!v.is_nan());
        NotNaN(v)
    }
}

impl<T: Float> Deref for NotNaN<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: Float + PartialEq> Eq for NotNaN<T> {}

/// An error indicating an attempt to construct NotNaN from a NaN
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FloatIsNaN;

impl Error for FloatIsNaN {
    fn description(&self) -> &str {
        return "NotNaN constructed with NaN"
    }
}

impl fmt::Display for FloatIsNaN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl Into<io::Error> for FloatIsNaN {
    fn into(self) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidInput, self)
    }
}

#[cfg(feature = "rustc-serialize")]
mod impl_rustc {
    extern crate rustc_serialize;
    use self::rustc_serialize::{Encodable, Encoder, Decodable, Decoder};
    use super::NotNaN;
    use std::error::Error;
    use num::Float;

    impl<T: Float + Encodable> Encodable for NotNaN<T> {
        fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
            self.0.encode(s)
        }
    }

    impl<T: Float + Decodable> Decodable for NotNaN<T> {
        fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
            T::decode(d).and_then(|v| NotNaN::new(v).map_err(|e| d.error(e.description())))
        }
    }
}

#[cfg(feature = "serde")]
mod impl_serde {
    extern crate serde;
    use self::serde::{Serialize, Serializer, Deserialize, Deserializer};
    use self::serde::de::Error;
    use super::NotNaN;
    use num::Float;

    impl<T: Float + Serialize> Serialize for NotNaN<T> {
        fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
            self.0.serialize(s)
        }
    }

    impl<T: Float + Deserialize> Deserialize for NotNaN<T> {
        fn deserialize<D: Deserializer>(d: &mut D) -> Result<Self, D::Error> {
            T::deserialize(d).and_then(|v| NotNaN::new(v).map_err(|_| <D::Error as Error>::syntax_error()))
        }
    }
}
