use super::Value;
use crate::{OwnedValue, Value as ValueTrait};

#[allow(clippy::cast_sign_loss, clippy::default_trait_access)]
impl<'v> PartialEq for Value<'v> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Static(s1), Self::Static(s2)) => s1 == s2,
            (Self::String(v1), Self::String(v2)) => v1.eq(v2),
            (Self::Array(v1), Self::Array(v2)) => v1.eq(v2),
            (Self::Object(v1), Self::Object(v2)) => v1.eq(v2),
            _ => false,
        }
    }
}

impl<'v, T> PartialEq<&T> for Value<'v>
where
    Value<'v>: PartialEq<T>,
{
    #[inline]
    fn eq(&self, other: &&T) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<OwnedValue> for Value<'a> {
    #[inline]
    fn eq(&self, other: &OwnedValue) -> bool {
        // We only need to implement this once
        other.eq(self)
    }
}

impl<'v> PartialEq<()> for Value<'v> {
    #[inline]
    fn eq(&self, _other: &()) -> bool {
        self.is_null()
    }
}

impl<'v> PartialEq<bool> for Value<'v> {
    #[inline]
    fn eq(&self, other: &bool) -> bool {
        self.as_bool().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<str> for Value<'v> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<&str> for Value<'v> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl<'v> PartialEq<String> for Value<'v> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.as_str().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<i8> for Value<'v> {
    #[inline]
    fn eq(&self, other: &i8) -> bool {
        self.as_i8().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<i16> for Value<'v> {
    #[inline]
    fn eq(&self, other: &i16) -> bool {
        self.as_i16().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<i32> for Value<'v> {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        self.as_i32().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<i64> for Value<'v> {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        self.as_i64().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<i128> for Value<'v> {
    #[inline]
    fn eq(&self, other: &i128) -> bool {
        self.as_i128().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<u8> for Value<'v> {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        self.as_u8().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<u16> for Value<'v> {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.as_u16().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<u32> for Value<'v> {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.as_u32().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<u64> for Value<'v> {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        self.as_u64().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<usize> for Value<'v> {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        self.as_usize().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<u128> for Value<'v> {
    #[inline]
    fn eq(&self, other: &u128) -> bool {
        self.as_u128().map(|t| t.eq(other)).unwrap_or_default()
    }
}

impl<'v> PartialEq<f32> for Value<'v> {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.as_f32().map(|t| t.eq(other)).unwrap_or_default()
    }
}
impl<'v> PartialEq<f64> for Value<'v> {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.as_f64().map(|t| t.eq(other)).unwrap_or_default()
    }
}
