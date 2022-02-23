use std::convert::From;

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ConversionError<S, T> {
    source: S,
    phantom: std::marker::PhantomData<T>,
}

impl<S, T> From<S> for ConversionError<S, T> {
    fn from(s: S) -> ConversionError<S, T> {
        ConversionError {
            source: s,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<S: fmt::Debug, T> fmt::Display for ConversionError<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConversionError: cannot convert {:?} to {}",
            self.source,
            std::any::type_name::<T>()
        )
    }
}

#[macro_export]
macro_rules! injective_from_pattern {
    ($T:ty => $V:ty, $p:path) => {
        impl std::convert::From<$T> for $V {
            fn from(x: $T) -> $V {
                $p(x)
            }
        }
        impl std::convert::TryFrom<$V> for $T {
            type Error = ConversionError<$V, $T>;
            fn try_from(x: $V) -> Result<$T, ConversionError<$V, $T>> {
                match x {
                    $p(v) => Ok(v),
                    other => Err(ConversionError::from(other)),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! injective_from_property {
    ($T:ty => $V:ty, $p:ident, $C:ty) => {
        impl std::convert::From<$T> for $V {
            fn from(x: $T) -> $V {
                let mut res = <$V>::default();
                res.$p = x.into();
                res
            }
        }
        impl std::convert::TryFrom<$V> for $T {
            type Error = <$T as std::convert::TryFrom<$C>>::Error;
            fn try_from(x: $V) -> Result<$T, Self::Error> {
                x.$p.try_into()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[derive(Debug, PartialEq)]
    enum MyType {
        Number(f64),
        NoNumber,
    }

    injective_from_pattern!(f64 => MyType, MyType::Number);

    #[test]
    fn injective_from_orig_is_sane() {
        assert_eq!(MyType::Number(2.), MyType::from(2.));
    }

    #[test]
    fn injective_wrong_pattern_conversion_error() {
        assert_eq!(f64::try_from(MyType::NoNumber), Err(ConversionError::from(MyType::NoNumber)));
    }

    #[test]
    fn injective_try_from_other_is_sane() {
        assert_eq!(f64::try_from(MyType::Number(2.)), Ok(2.));
    }
}
