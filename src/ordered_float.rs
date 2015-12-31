use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::fmt;
use num::Float;

/// A wrapper around Floats providing an implementation of Ord and Hash.
///
/// NaN is sorted as *greater* than all other values and *equal*
/// to itself, in contradiction with the IEEE standard.
#[derive(PartialOrd, Debug, Default, Clone, Copy)]
pub struct OrderedFloat<T: Float>(pub T);

impl<T: Float> OrderedFloat<T> {
    /// Get the value out.
    pub fn into_inner(self) -> T {
        let OrderedFloat(val) = self;
        val
    }
}

impl<T: Float> AsRef<T> for OrderedFloat<T> {
    fn as_ref(&self) -> &T {
        let OrderedFloat(ref val) = *self;
        val
    }
}

impl<T: Float> AsMut<T> for OrderedFloat<T> {
    fn as_mut(&mut self) -> &mut T {
        let OrderedFloat(ref mut val) = *self;
        val
    }
}

impl<T: Float + Copy + PartialOrd> Ord for OrderedFloat<T> {
    fn cmp(&self, other: &OrderedFloat<T>) -> Ordering {
        match self.partial_cmp(&other) {
            Some(ordering) => ordering,
            None => {
                if self.as_ref().is_nan() {
                    if other.as_ref().is_nan() {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

impl<T: Float + PartialEq> PartialEq for OrderedFloat<T> {
    fn eq(&self, other: &OrderedFloat<T>) -> bool {
        if self.as_ref().is_nan() {
            if other.as_ref().is_nan() {
                true
            } else {
                false
            }
        } else if other.as_ref().is_nan() {
            false
        } else {
            self.as_ref() == other.as_ref()
        }
    }
}

impl<T: Float> Hash for OrderedFloat<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (man, exp, sign) = if self.as_ref().is_nan() {
            // normalize to one representation of NaN
            T::nan().integer_decode()
        } else {
            self.as_ref().integer_decode()
        };
        (man ^ exp as u64 ^ sign as u64).hash(state)
    }
}

impl<T: Float + fmt::Display> fmt::Display for OrderedFloat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl Into<f32> for OrderedFloat<f32> {
    fn into(self) -> f32 {
        self.into_inner()
    }
}

impl Into<f64> for OrderedFloat<f64> {
    fn into(self) -> f64 {
        self.into_inner()
    }
}

impl<T: Float> From<T> for OrderedFloat<T> {
    fn from(val: T) -> Self {
        OrderedFloat(val)
    }
}

impl<T: Float> Deref for OrderedFloat<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: Float> DerefMut for OrderedFloat<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: Float + PartialEq> Eq for OrderedFloat<T> { }

#[cfg(feature = "rustc-serialize")]
mod impl_rustc {
    extern crate rustc_serialize;
    use self::rustc_serialize::{Encodable, Encoder, Decodable, Decoder};
    use super::OrderedFloat;
    use std::error::Error;
    use num::Float;

    impl<T: Float + Encodable> Encodable for OrderedFloat<T> {
        fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
            self.0.encode(s)
        }
    }

    impl<T: Float + Decodable> Decodable for OrderedFloat<T> {
        fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
            T::decode(d).map(OrderedFloat)
        }
    }
}

#[cfg(feature = "serde")]
mod impl_serde {
    extern crate serde;
    use self::serde::{Serialize, Serializer, Deserialize, Deserializer};
    use self::serde::de::Error;
    use super::OrderedFloat;
    use num::Float;

    impl<T: Float + Serialize> Serialize for OrderedFloat<T> {
        fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
            self.0.serialize(s)
        }
    }

    impl<T: Float + Deserialize> Deserialize for OrderedFloat<T> {
        fn deserialize<D: Deserializer>(d: &mut D) -> Result<Self, D::Error> {
            T::deserialize(d).map(OrderedFloat)
        }
    }
}
