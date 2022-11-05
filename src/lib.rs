#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(target_os = "linux"))]
compile_error!("syslib only supports Linux");

mod sys;
pub use sys::*;

pub mod sock;

pub const NUMBER_BITS: u32 = 8;
pub const NUMBER_SHIFT: u32 = 0;
pub const NUMBER_MASK: u32 = (1 << NUMBER_BITS) - 1;

pub const CODE_BITS: u32 = 8;
pub const CODE_SHIFT: u32 = NUMBER_BITS + NUMBER_SHIFT;
pub const CODE_MASK: u32 = (1 << CODE_BITS) - 1;

pub const SIZE_BITS: u32 = 14;
pub const SIZE_SHIFT: u32 = CODE_BITS + CODE_SHIFT;
pub const SIZE_MASK: u32 = (1 << SIZE_BITS) - 1;

pub const DIRECTION_BITS: u32 = 2;
pub const DIRECTION_SHIFT: u32 = SIZE_BITS + SIZE_SHIFT;
pub const DIRECTION_MASK: u32 = (1 << DIRECTION_BITS) - 1;

#[macro_export]
macro_rules! ioctl {
    ($direction:expr, $code:expr, $number:expr, $ty:ty) => {
        (($number & $crate::NUMBER_MASK) << $crate::NUMBER_SHIFT) |
        (($code & $crate::CODE_MASK) << $crate::CODE_SHIFT) |
        ((::core::mem::size_of::<$ty>() as u32 & $crate::SIZE_MASK) << $crate::SIZE_SHIFT) |
        (($direction & $crate::DIRECTION_MASK) << $crate::DIRECTION_SHIFT)
    };
    ($code:expr, $number:expr, $ty:ty) => {
        ioctl!(0, $code, $number, $ty)
    };
    (write; $code:expr, $number:expr, $ty:ty) => {
        ioctl!(1, $code, $number, $ty)
    };
    (read; $code:expr, $number:expr, $ty:ty) => {
        ioctl!(2, $code, $number, $ty)
    };
    (read + write; $code:expr, $number:expr, $ty:ty) => {
        ioctl!(1 | 2, $code, $number, $ty)
    };
}

/// Create a file access mode.
/// Roughly follows the format shown by `ls -l`.
/// ```rust
/// use syslib::mode;
/// assert_eq!(mode!(___ ___ ___), syslib::open::Mode(0o000));
/// assert_eq!(mode!(rwx r_x ___), syslib::open::Mode(0o750));
/// assert_eq!(mode!(rwxS r_xS r_xT), syslib::open::Mode(0o7755));
/// ```
#[macro_export]
macro_rules! mode {
    ($user:ident $group:ident $other:ident) => {
        // Wrapped so that we can add a doc test
        syslib_macro::mode!($user $group $other)
    };
}

#[macro_export]
macro_rules! c_flags {
    (
        $vis:vis $name:ident($ty:ty) {
            $($item:ident = $value:expr),*
        } $catch:pat => $return:expr
    ) => {
        #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq, ::core::cmp::Eq)]
        #[repr(transparent)]
        $vis struct $name($vis $ty);
        impl $name {
            $vis const NONE: Self = Self(0);
            $(
                $vis const $item: Self = Self($value);
            )*
            $vis const MASK: Self = Self($($value|)* 0);
            /// Returns true if any of the bits are set
            pub fn any(self, bits: Self) -> bool {
                self & bits != Self::NONE
            }
            /// Returns true if all of the bits are set
            pub fn all(self, bits: Self) -> bool {
                self & bits == bits
            }
        }
        impl ::core::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }
        impl ::core::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0
            }
        }
        impl ::core::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }
        impl ::core::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0
            }
        }
        impl ::core::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }
        impl ::core::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0
            }
        }
        impl ::core::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #[derive(Debug)]
                struct Unknown($ty);
                $(
                    #[derive(Debug)]
                    #[allow(non_camel_case_types)]
                    struct $item;
                )*
                let mut list = f.debug_tuple(stringify!($name));
                $(if self.all(Self($value)) { list.field(&$item); })*
                let invalid = *self & !Self::MASK;
                if invalid != Self::NONE {
                    list.field(&Unknown(self.0));
                }
                list.finish()
            }
        }
        impl ::core::convert::TryFrom<$ty> for $name {
            type Error = $crate::Error;
            fn try_from(value: $ty) -> ::core::result::Result<Self, Self::Error> {
                match Self(value) & !Self::MASK {
                    Self::NONE => Ok(Self(value)),
                    $catch => $return
                }
            }
        }
        impl ::core::convert::From<$name> for $ty {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    }
}

#[macro_export]
macro_rules! enumeration {
    (
        $vis:vis struct $ident:ident($ty:ty) {
            $(
                #[$display:literal]
                $item:ident = $value:literal
            ),*
        }
    ) => {
        #[non_exhaustive]
        #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq, ::core::cmp::Eq)]
        #[repr(transparent)]
        $vis struct $ident($ty);
        impl $ident {
            $(pub const $item: Self = Self($value);)*
        }

        impl ::core::fmt::Debug for $ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match *self {
                    $(
                        Self::$item => ::core::write!(f, "{}({})", ::core::stringify!($item), $value),
                    )*
                    e => ::core::write!(f, "UNKNOWN({})", e.0)
                }
            }
        }

        impl ::core::fmt::Display for $ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match *self {
                    $(
                        Self::$item => f.write_str($display),
                    )*
                    _ => f.write_str("Unknown enumeration value.")
                }
            }
        }

        impl ::core::convert::Into<$ty> for $ident {
            fn into(self) -> $ty {
                self.0
            }
        }
        impl ::core::convert::From<$ty> for $ident {
            fn from(value: $ty) -> Self {
                Self(value)
            }
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_write() {
        use crate::sys::*;
        write(Fd::stdout, format!("{:#?}", stat("/dev/null").unwrap()).as_bytes()).unwrap();
    }
}